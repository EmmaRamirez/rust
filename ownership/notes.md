# Ownership

> Rust makes two promises
- You decide the lifetime of each value in your program
- Even so, your program will never use a pointer to an object after it has been freed

## About Ownership

```c
std::string s = "frayed knot";
```

The string `s` is represented in memory as a pointer to a heap-allocated buffer.

A std::string owns its buffers: when the program destroys the string, the string's destructor frees the buffer.