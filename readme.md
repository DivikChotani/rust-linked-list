# Notes: Case-by-Case Analysis of Unsafe Pointer/Reference Variants

Below is a breakdown of each code variant, the observed Miri behavior, and why it does or does not satisfy the Stacked Borrows rules.

---

## Case 1: Shared borrow **before** writes (fails)

```rust
unsafe {
    let mut data = 10;
    let mref1 = &mut data;
    let ptr2 = mref1 as *mut i32;
    let sref3 = &*mref1;        // SharedReadOnly tag

    *ptr2 += 2;                // Invalidate shared tag
    opaque_read(sref3);        // ERROR: tag no longer on stack
}
```

**Outcome:** Miri reports Undefined Behavior at `opaque_read(sref3)` because the shared tag was invalidated by the write (`*ptr2 += 2`).

**Why it fails:** Stacked Borrows requires that a shared borrow’s tag remain on the stack for later reads. The mutable write pops that tag, so subsequent shared access is invalid.

---

## Case 2: No shared borrow at all (works)

```rust
unsafe {
    let mut data = 10;
    let mref1 = &mut data;
    let ptr2 = mref1 as *mut i32;
    // let sref3 = &*mref1;    // commented out

    *ptr2 += 2;
    opaque_read(&*ptr2 as &i32);  // reborrow at read-time
}
```

**Outcome:** Prints `12` with no errors.

**Why it works:** No shared tag is ever created before the write. When you do `opaque_read(&*ptr2)`, you reborrow **after** all mutations, so the new shared borrow is valid.

---

## Case 3: Casting via `sref3` then writing (fails)

```rust
unsafe {
    let mut data = 10;
    let mref1 = &mut data;
    let sref3 = &*mref1;
    let p4    = sref3 as *const i32;
    let ptr2  = mref1 as *mut i32;

    *ptr2 += 2;         // invalidates SharedReadOnly tag <2706>
    opaque_read(&*p4);  // ERROR: original tag gone
}
```

**Outcome:** Miri errors at the `opaque_read(&*p4)` retag.

**Why it fails:** Raw pointer `p4` inherits the shared tag from `sref3`. The write through `ptr2` pops that tag, so `p4`’s subsequent read is illegal.

---

## Case 4: Read **before** write, then write, then read again (works)

```rust
unsafe {
    let mut data = 10;
    let ptr   = &mut data as *mut i32;

    opaque_read(&*ptr);   // tag created and used here (prints 10)
    *ptr += 2;            // permitted
    opaque_read(&*ptr);   // reuses mutable tag (prints 12)
}
```

**Outcome:** No errors; prints `10` then `12`.

**Why it works:** The first read creates a shared tag and immediately uses it before any writes. After that read, the mutable borrow tag remains active, so later writes and reads through the mutable tag are fine.

---

## Summary

* **Shared borrows must outlive no intervening writes** on the same location.
* **Raw pointers inherit the tag** from the reference they were cast from.
* To avoid retag errors:

  1. **Reborrow only after** all writes are done.
  2. **Drop/narrow** any shared reference before performing writes.
  3. **Perform reads early** if you need to observe the pre-write value.

These rules ensure your unsafe code stays within Miri’s Stacked Borrows model and prevents accidental aliasing violations.
