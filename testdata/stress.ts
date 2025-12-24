import { EventEmitter } from 'events';

// ============================================================================
// TYPE DEFINITIONS
// ============================================================================

// Simple interface
interface User {
  id: number;
  name: string;
  email: string;
}

// Generic interface
interface Repository<T, ID = number> {
  findById(id: ID): Promise<T | null>;
  findAll(): Promise<T[]>;
  save(entity: T): Promise<T>;
  delete(id: ID): Promise<boolean>;
}

// Interface with optional and readonly
interface ConfigOptions {
  readonly apiKey: string;
  baseUrl?: string;
  timeout: number;
  retries?: number;
  headers?: Record<string, string>;
  onError?: (error: Error) => void;
}

// Interface extending multiple interfaces
interface AuditableEntity {
  createdAt: Date;
  updatedAt: Date;
  createdBy: string;
}

interface SoftDeletable {
  deletedAt?: Date;
  isDeleted: boolean;
}

interface AuditedUser extends User, AuditableEntity, SoftDeletable {
  lastLoginAt?: Date;
  loginCount: number;
}

// Type alias with complex union
type Result<T, E = Error> =
  | { success: true; data: T }
  | { success: false; error: E };

// Mapped type
type Readonly<T> = {
  readonly [P in keyof T]: T[P];
};

// Conditional type
type NonNullable<T> = T extends null | undefined ? never : T;

// Template literal type
type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
type ApiEndpoint<M extends HttpMethod> = `${Lowercase<M>}:${string}`;

// ============================================================================
// SIMPLE FUNCTIONS
// ============================================================================

// Simple function
function simpleFunction(): number {
  return 42;
}

// Function with generics
function identity<T>(value: T): T {
  return value;
}

// Function with complex generics
function mapObject<T extends object, K extends keyof T, V>(
  obj: T,
  key: K,
  transform: (value: T[K]) => V
): V {
  return transform(obj[key]);
}

// Arrow function (const)
const arrowFunction = (x: number, y: number): number => {
  return x + y;
};

// Arrow function with generics
const genericArrow = <T extends { id: number }>(items: T[]): Map<number, T> => {
  return new Map(items.map((item) => [item.id, item]));
};

// Async function
async function fetchUser(id: number): Promise<User | null> {
  const response = await fetch(`/api/users/${id}`);
  if (!response.ok) return null;
  return response.json();
}

// Async arrow function
const fetchUsers = async (): Promise<User[]> => {
  const response = await fetch('/api/users');
  return response.json();
};

// Generator function
function* numberGenerator(start: number, end: number): Generator<number> {
  for (let i = start; i <= end; i++) {
    yield i;
  }
}

// Async generator
async function* asyncDataStream<T>(
  source: AsyncIterable<T>
): AsyncGenerator<T, void, unknown> {
  for await (const item of source) {
    yield item;
  }
}

// Function overloads
function processValue(value: string): string;
function processValue(value: number): number;
function processValue(value: string | number): string | number {
  if (typeof value === 'string') {
    return value.toUpperCase();
  }
  return value * 2;
}

// Function with rest params and destructuring
function mergeConfigs(
  base: ConfigOptions,
  ...overrides: Partial<ConfigOptions>[]
): ConfigOptions {
  return overrides.reduce(
    (acc, override) => ({ ...acc, ...override }),
    { ...base }
  );
}

// Higher-order function
function createMultiplier(factor: number): (value: number) => number {
  return (value: number) => value * factor;
}

// Function with callback
function withRetry<T>(
  fn: () => Promise<T>,
  options: { retries: number; delay: number }
): Promise<T> {
  const { retries, delay } = options;

  return new Promise(async (resolve, reject) => {
    for (let attempt = 0; attempt <= retries; attempt++) {
      try {
        const result = await fn();
        resolve(result);
        return;
      } catch (error) {
        if (attempt === retries) {
          reject(error);
        }
        await new Promise((r) => setTimeout(r, delay));
      }
    }
  });
}

// ============================================================================
// CLASSES
// ============================================================================

// Simple class
class SimpleClass {
  private value: number;

  constructor(value: number) {
    this.value = value;
  }

  getValue(): number {
    return this.value;
  }
}

// Abstract class
abstract class BaseEntity<ID = number> {
  abstract id: ID;
  abstract validate(): boolean;

  protected createdAt: Date = new Date();

  getCreatedAt(): Date {
    return this.createdAt;
  }
}

// Generic class with constraints
class GenericRepository<T extends { id: number }> implements Repository<T> {
  private items: Map<number, T> = new Map();

  async findById(id: number): Promise<T | null> {
    return this.items.get(id) ?? null;
  }

  async findAll(): Promise<T[]> {
    return Array.from(this.items.values());
  }

  async save(entity: T): Promise<T> {
    this.items.set(entity.id, entity);
    return entity;
  }

  async delete(id: number): Promise<boolean> {
    return this.items.delete(id);
  }
}

// Class with decorators (decorator syntax)
function logged(target: any, key: string, descriptor: PropertyDescriptor) {
  const original = descriptor.value;
  descriptor.value = function (...args: any[]) {
    console.log(`Calling ${key} with`, args);
    return original.apply(this, args);
  };
  return descriptor;
}

function injectable(constructor: Function) {
  // DI container registration logic
}

// Complex class with everything
class ComplexService extends EventEmitter {
  private static instance: ComplexService | null = null;
  private readonly config: ConfigOptions;
  private cache: Map<string, unknown> = new Map();
  #privateField: number = 0; // ES2022 private field

  // Static factory method
  static getInstance(config: ConfigOptions): ComplexService {
    if (!ComplexService.instance) {
      ComplexService.instance = new ComplexService(config);
    }
    return ComplexService.instance;
  }

  private constructor(config: ConfigOptions) {
    super();
    this.config = config;
  }

  // Getter
  get cacheSize(): number {
    return this.cache.size;
  }

  // Setter
  set maxRetries(value: number) {
    if (value < 0) throw new Error('maxRetries must be non-negative');
    // @ts-ignore - for testing
    this.config.retries = value;
  }

  // Async method
  async fetchData<T>(endpoint: string): Promise<Result<T>> {
    try {
      const url = `${this.config.baseUrl}${endpoint}`;
      const response = await fetch(url, {
        headers: {
          'Authorization': `Bearer ${this.config.apiKey}`,
          ...this.config.headers,
        },
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const data = await response.json();
      return { success: true, data };
    } catch (error) {
      this.config.onError?.(error as Error);
      return { success: false, error: error as Error };
    }
  }

  // Method with complex types
  processItems<T extends object>(
    items: T[],
    processor: (item: T, index: number) => T | null
  ): T[] {
    return items
      .map((item, index) => processor(item, index))
      .filter((item): item is T => item !== null);
  }

  // Private method
  private validateConfig(): boolean {
    return !!(this.config.apiKey && this.config.timeout > 0);
  }

  // Protected method
  protected emitEvent(event: string, data: unknown): void {
    this.emit(event, data);
  }

  // Static method
  static createDefaultConfig(): ConfigOptions {
    return {
      apiKey: '',
      baseUrl: 'https://api.example.com',
      timeout: 5000,
      retries: 3,
    };
  }

  // Method using private field
  incrementPrivate(): number {
    return ++this.#privateField;
  }
}

// Class with nested class
class OuterClass {
  private data: string;

  constructor(data: string) {
    this.data = data;
  }

  // Nested class pattern via static property
  static Inner = class {
    process(value: string): string {
      return value.toUpperCase();
    }
  };

  createInner() {
    return new OuterClass.Inner();
  }
}

// Mixin pattern
type Constructor<T = {}> = new (...args: any[]) => T;

function Timestamped<TBase extends Constructor>(Base: TBase) {
  return class extends Base {
    timestamp = new Date();

    getTimestamp(): Date {
      return this.timestamp;
    }
  };
}

function Tagged<TBase extends Constructor>(Base: TBase) {
  return class extends Base {
    tags: string[] = [];

    addTag(tag: string): void {
      this.tags.push(tag);
    }
  };
}

// Class using mixins
class Document {
  constructor(public title: string) {}
}

const TimestampedDocument = Timestamped(Document);
const TaggedTimestampedDocument = Tagged(Timestamped(Document));

// ============================================================================
// EDGE CASES
// ============================================================================

// Function with template literal in body
function buildQuery(table: string, conditions: Record<string, unknown>): string {
  const whereClause = Object.entries(conditions)
    .map(([key, value]) => `${key} = '${value}'`)
    .join(' AND ');
  return `SELECT * FROM ${table} WHERE ${whereClause}`;
}

// Interface with index signature
interface DynamicObject {
  [key: string]: unknown;
  id: number;
  type: string;
}

// Interface with call signature
interface Callable {
  (arg: string): number;
  displayName: string;
}

// Interface with construct signature
interface Constructable<T> {
  new (arg: string): T;
  prototype: T;
}

// Deeply nested generic types
interface DeepNested<
  A extends object,
  B extends keyof A,
  C extends A[B] extends object ? keyof A[B] : never
> {
  get<K extends B>(key: K): A[K];
  getDeep<K1 extends B, K2 extends C>(key1: K1, key2: K2): unknown;
}

// Function with complex destructuring
function complexDestructuring({
  user: { name, email },
  settings: { theme = 'light', ...otherSettings },
  items = [],
}: {
  user: User;
  settings: { theme?: string; [key: string]: unknown };
  items?: string[];
}): { displayName: string; config: object } {
  return {
    displayName: `${name} <${email}>`,
    config: { theme, ...otherSettings, itemCount: items.length },
  };
}

// Very long function signature
function veryLongFunctionWithManyParameters(
  firstParameter: string,
  secondParameter: number,
  thirdParameter: boolean,
  fourthParameter: Date,
  fifthParameter: User,
  sixthParameter: ConfigOptions,
  seventhParameter?: Partial<AuditedUser>,
  eighthParameter?: (value: unknown) => boolean
): Promise<{
  processed: boolean;
  results: Array<{
    id: number;
    status: 'success' | 'failure' | 'pending';
    data?: unknown;
    error?: Error;
  }>;
  metadata: {
    startTime: Date;
    endTime: Date;
    duration: number;
  };
}> {
  const startTime = new Date();

  return new Promise((resolve) => {
    setTimeout(() => {
      const endTime = new Date();
      resolve({
        processed: true,
        results: [],
        metadata: {
          startTime,
          endTime,
          duration: endTime.getTime() - startTime.getTime(),
        },
      });
    }, 100);
  });
}

// Export patterns
export function exportedFunction(): void {}

export const exportedArrow = (): void => {};

export class ExportedClass {
  method(): void {}
}

export interface ExportedInterface {
  field: string;
}

export default class DefaultExportClass {
  value: number = 0;

  increment(): number {
    return ++this.value;
  }
}

// Function at end of file
function finalFunction(): string {
  return "I'm at the end!";
}
