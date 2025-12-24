# Stress test file for sread - Python
# Contains edge cases, nested structures, decorators, async, etc.

import asyncio
from typing import Generic, TypeVar, Optional, List, Dict, Callable, Union
from dataclasses import dataclass
from functools import wraps
from contextlib import contextmanager

T = TypeVar('T')
U = TypeVar('U')

# Simple function
def simple_function():
    """A simple function."""
    return 42

# Function with complex signature
def complex_signature(
    arg1: str,
    arg2: int = 10,
    *args: tuple,
    keyword_only: bool = False,
    **kwargs: dict
) -> Optional[Dict[str, List[int]]]:
    """Function with complex type hints and signature."""
    result = {arg1: [arg2] * len(args)}
    if keyword_only:
        result.update(kwargs)
    return result

# Decorated function
def my_decorator(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@my_decorator
def decorated_function(x: int, y: int) -> int:
    """This function is decorated."""
    return x + y

# Multiple decorators
@staticmethod
@my_decorator
def multi_decorated():
    pass

# Async function
async def fetch_data(url: str) -> dict:
    """Async function to fetch data."""
    await asyncio.sleep(0.1)
    return {"url": url, "data": "sample"}

# Async generator
async def async_generator(n: int):
    """Async generator yielding numbers."""
    for i in range(n):
        await asyncio.sleep(0.01)
        yield i

# Generator function
def fibonacci_generator(limit: int):
    """Generator that yields Fibonacci numbers."""
    a, b = 0, 1
    while a < limit:
        yield a
        a, b = b, a + b

# Context manager function
@contextmanager
def managed_resource(name: str):
    """Context manager for a resource."""
    print(f"Acquiring {name}")
    try:
        yield {"resource": name}
    finally:
        print(f"Releasing {name}")

# Lambda stored in variable (should not be extracted as function)
square = lambda x: x ** 2

# Nested function
def outer_function(x: int) -> Callable[[int], int]:
    """Function containing nested function."""

    def inner_function(y: int) -> int:
        """This is the inner function."""
        return x + y

    return inner_function

# Function with multiline string
def function_with_docstring():
    """
    This is a very long docstring that spans
    multiple lines and contains:

    - Bullet points
    - Code examples:
        >>> function_with_docstring()
        None
    - And more text

    Returns:
        None
    """
    pass

# Class with everything
class ComplexClass(Generic[T, U]):
    """A complex class with generics, nested classes, and various methods."""

    class_variable: int = 100
    _private_class_var: str = "private"

    class NestedClass:
        """A nested class inside ComplexClass."""

        def nested_method(self) -> str:
            return "I'm nested"

        class DeeplyNestedClass:
            """Even more deeply nested."""

            def deeply_nested_method(self):
                return "Very deep"

    def __init__(self, value: T, other: U) -> None:
        """Initialize the complex class."""
        self.value = value
        self.other = other
        self._cache: Dict[str, any] = {}

    def regular_method(self, arg: str) -> str:
        """A regular instance method."""
        return f"{self.value}: {arg}"

    @property
    def computed_property(self) -> str:
        """A property that computes something."""
        return f"Computed: {self.value}"

    @computed_property.setter
    def computed_property(self, value: str) -> None:
        """Setter for the property."""
        self._cache['computed'] = value

    @classmethod
    def from_string(cls, s: str) -> 'ComplexClass[str, str]':
        """Class method factory."""
        parts = s.split(',')
        return cls(parts[0], parts[1] if len(parts) > 1 else "")

    @staticmethod
    def utility_function(x: int, y: int) -> int:
        """A static utility method."""
        return x * y

    async def async_method(self, delay: float) -> T:
        """An async method."""
        await asyncio.sleep(delay)
        return self.value

    def method_with_lambda(self) -> List[int]:
        """Method that uses lambdas internally."""
        numbers = [1, 2, 3, 4, 5]
        return list(map(lambda x: x * 2, filter(lambda x: x % 2 == 0, numbers)))

    def __repr__(self) -> str:
        return f"ComplexClass({self.value!r}, {self.other!r})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, ComplexClass):
            return NotImplemented
        return self.value == other.value and self.other == other.other

    def __hash__(self) -> int:
        return hash((self.value, self.other))

    def __enter__(self) -> 'ComplexClass[T, U]':
        return self

    def __exit__(self, exc_type, exc_val, exc_tb) -> bool:
        return False

# Dataclass
@dataclass
class DataClassExample:
    """A dataclass example."""
    name: str
    value: int
    tags: List[str] = None

    def __post_init__(self):
        if self.tags is None:
            self.tags = []

    def add_tag(self, tag: str) -> None:
        """Add a tag to this instance."""
        self.tags.append(tag)

# Abstract base class pattern
class AbstractProcessor:
    """Abstract base class for processors."""

    def process(self, data: dict) -> dict:
        """Process the data - must be overridden."""
        raise NotImplementedError("Subclasses must implement process()")

    def validate(self, data: dict) -> bool:
        """Validate input data."""
        return isinstance(data, dict) and len(data) > 0

class ConcreteProcessor(AbstractProcessor):
    """Concrete implementation of processor."""

    def __init__(self, multiplier: int = 1):
        self.multiplier = multiplier

    def process(self, data: dict) -> dict:
        """Process by multiplying all numeric values."""
        result = {}
        for key, value in data.items():
            if isinstance(value, (int, float)):
                result[key] = value * self.multiplier
            else:
                result[key] = value
        return result

# Mixin class
class LoggingMixin:
    """Mixin that adds logging capability."""

    def log(self, message: str, level: str = "INFO") -> None:
        """Log a message."""
        print(f"[{level}] {self.__class__.__name__}: {message}")

    def log_error(self, error: Exception) -> None:
        """Log an error."""
        self.log(str(error), level="ERROR")

# Multiple inheritance
class LoggingProcessor(ConcreteProcessor, LoggingMixin):
    """Processor with logging capability."""

    def process(self, data: dict) -> dict:
        """Process with logging."""
        self.log(f"Processing {len(data)} items")
        try:
            result = super().process(data)
            self.log(f"Processed successfully")
            return result
        except Exception as e:
            self.log_error(e)
            raise

# Exception class
class CustomError(Exception):
    """Custom exception for the module."""

    def __init__(self, message: str, code: int = 0):
        super().__init__(message)
        self.code = code

    def __str__(self) -> str:
        return f"CustomError[{self.code}]: {super().__str__()}"

# Metaclass example
class SingletonMeta(type):
    """Metaclass for singleton pattern."""

    _instances: Dict[type, object] = {}

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super().__call__(*args, **kwargs)
        return cls._instances[cls]

class SingletonClass(metaclass=SingletonMeta):
    """A singleton class using metaclass."""

    def __init__(self, value: int = 0):
        self.value = value

# Protocol (structural subtyping)
from typing import Protocol, runtime_checkable

@runtime_checkable
class Drawable(Protocol):
    """Protocol for drawable objects."""

    def draw(self) -> None:
        """Draw the object."""
        ...

    def get_bounds(self) -> tuple:
        """Get bounding box."""
        ...

# Function at end of file with colon in string (edge case)
def tricky_function():
    """Function with colons: in docstring: and more: colons."""
    data = {"key:with:colons": "value:also:colons"}
    pattern = r"regex:pattern:\d+"
    return f"result: {data}"

# Very long function
def very_long_function_with_lots_of_logic(
    input_data: List[Dict[str, Union[int, str, List[float]]]],
    config: Optional[Dict[str, any]] = None,
    *,
    validate: bool = True,
    transform: bool = True,
    aggregate: bool = False
) -> Dict[str, Union[int, float, List[str]]]:
    """
    A very long function that does many things.

    This function processes input data through multiple stages:
    1. Validation
    2. Transformation
    3. Aggregation (optional)

    Args:
        input_data: List of dictionaries to process
        config: Optional configuration dictionary
        validate: Whether to validate input
        transform: Whether to transform data
        aggregate: Whether to aggregate results

    Returns:
        Processed data dictionary
    """
    if config is None:
        config = {
            "max_items": 1000,
            "default_value": 0,
            "string_transform": str.upper,
            "numeric_transform": lambda x: x * 2,
        }

    # Validation stage
    if validate:
        if not isinstance(input_data, list):
            raise TypeError("input_data must be a list")
        if len(input_data) > config.get("max_items", 1000):
            raise ValueError(f"Too many items: {len(input_data)}")
        for i, item in enumerate(input_data):
            if not isinstance(item, dict):
                raise TypeError(f"Item {i} is not a dictionary")

    # Transformation stage
    result: Dict[str, Union[int, float, List[str]]] = {
        "count": len(input_data),
        "strings": [],
        "numbers": [],
        "total": 0,
    }

    if transform:
        string_transform = config.get("string_transform", str)
        numeric_transform = config.get("numeric_transform", lambda x: x)

        for item in input_data:
            for key, value in item.items():
                if isinstance(value, str):
                    result["strings"].append(string_transform(value))
                elif isinstance(value, (int, float)):
                    transformed = numeric_transform(value)
                    result["numbers"].append(transformed)
                    result["total"] += transformed
                elif isinstance(value, list):
                    for v in value:
                        if isinstance(v, (int, float)):
                            result["total"] += numeric_transform(v)

    # Aggregation stage
    if aggregate:
        if result["numbers"]:
            result["average"] = result["total"] / len(result["numbers"])
            result["min"] = min(result["numbers"])
            result["max"] = max(result["numbers"])
        result["unique_strings"] = list(set(result["strings"]))

    return result

# Final simple function to ensure we can extract things at end of file
def final_function() -> str:
    """The last function in the file."""
    return "I'm at the end!"
