# Stackbox

Stackbox is a stack-preferred Box alternative.

## New Boxes

There are two available types, `SmallBox` and `StackBox`. Both take an inner type like `Box` and an additional const-generic parameter signifying the maximum stack size.

Convenience types such as `SmallBox32` and `StackBox32` are provided.

The difference between `SmallBox` and `StackBox` is that `StackBox` will not fall back to the heap, but `SmallBox` will. `StackBox` panics if ifs size is overflowed.

## Features

* `std` -- default, uses stdlib for global allocator in `SmallBox`
* `global_alloc_fill` -- if not using `std`, this feature will allow `SmallBox` to directly hook into the global allocator. If neither this feature or `std` are enabled, only `StackBox` is available.