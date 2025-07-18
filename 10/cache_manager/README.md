Understood! Your desire for more practice is the mark of a true learner. The previous challenge honed your skills in combining generics, traits, and lifetimes for a data pipeline. Now, let's explore these concepts through a different lens: building a **Generic Cache with Pluggable Eviction Policies**.

This challenge will emphasize:

* **Generics (`<K, V>`):** To make your cache work with any key and value types.
* **Traits:** To define a flexible interface for different cache eviction strategies.
* **Trait Objects (`Box<dyn Trait>`):** To allow your cache to use *any* eviction policy that implements the trait, chosen at runtime. This introduces the concept of dynamic dispatch, which is a powerful application of traits.

---

### **Sensei's Dire Challenge: The Pluggable Cache (Chapter 10)**

Your task is to implement a generic `Cache` struct that can store key-value pairs and delegate its eviction logic to a separate, customizable "Eviction Policy" via a trait object.

**Core Requirements:**

1.  **Eviction Policy Trait (`EvictionPolicy<K, V>`):**
    * Define a public trait `EvictionPolicy<K, V>`.
    * This trait should require the following methods:
        * `on_access(&mut self, key: &K)`: Called by the cache when an item with `key` is retrieved. The policy can use this to update its internal state (e.g., for LRU).
        * `on_insert(&mut self, key: K)`: Called by the cache when a new item with `key` is inserted. The policy can use this to record the insertion order/frequency. **Note: This method takes `K` by value because the cache will pass ownership of the key to the policy.**
        * `evict(&mut self) -> Option<K>`: Called by the cache when it needs to make space. The policy should return `Some(key)` of the item that should be removed from the cache, or `None` if no eviction is needed/possible (e.g., if the cache is not full yet according to its internal logic).

2.  **Concrete Eviction Policy Implementations:**
    * Implement **at least two** distinct structs that implement `EvictionPolicy<K, V>`:
        * **`FifoPolicy<K>` (First-In, First-Out):**
            * This struct should use an internal `std::collections::VecDeque<K>` to keep track of the order of inserted keys.
            * `on_access` will do nothing (FIFO doesn't care about access).
            * `on_insert` will push the key to the back of the `VecDeque`.
            * `evict` will pop a key from the front of the `VecDeque`.
            * *Constraint:* `K` must implement `Debug`, `Clone`, and `Eq` (for potential internal checks, though `Eq` isn't strictly necessary for FIFO's core logic).
        * **`LruPolicy<K>` (Least Recently Used):**
            * This struct should use an internal `std::collections::LinkedList<K>` (or `VecDeque<K>`) to track access order. When a key is accessed, it's moved to the back of the list.
            * `on_access` will remove the `key` from its current position and add it to the back.
            * `on_insert` will add the key to the back of the list.
            * `evict` will remove a key from the front of the list.
            * *Constraint:* `K` must implement `Debug`, `Clone`, and `Eq`.

3.  **Generic Cache Struct (`Cache<K, V>`):**
    * Define a public generic struct `Cache<K, V>`.
    * It should have the following fields:
        * `data: std::collections::HashMap<K, V>`
        * `capacity: usize`
        * `policy: Box<dyn EvictionPolicy<K, V>>` (This is where the trait object comes in!)
    * Implement `Cache<K, V>`:
        * `new(capacity: usize, policy: Box<dyn EvictionPolicy<K, V>>) -> Self`: Constructor.
        * `insert(&mut self, key: K, value: V) -> Option<V>`:
            * If `self.data.len() >= self.capacity`, call `self.policy.evict()`.
            * If `evict` returns `Some(key_to_evict)`, remove that `key_to_evict` from `self.data`.
            * Insert the new `key` and `value` into `self.data`.
            * Call `self.policy.on_insert(key)` (remember `on_insert` takes ownership of `K`, so you might need to `clone` `key` before inserting it into `self.data`).
            * Return `Option<V>` (e.g., `None` or `Some(old_value)` if `key` already existed).
        * `get(&mut self, key: &K) -> Option<&V>`:
            * If the `key` is found in `self.data`, call `self.policy.on_access(key)`.
            * Return `Some(&V)` if found, `None` otherwise.
    * *Constraints:* `K` must implement `Debug`, `Clone`, `Eq`, and `Hash`. `V` must implement `Debug`.

4.  **`main` Function Demonstration:**
    * In your `main` function, demonstrate the use of your `Cache`:
        * Create a `FifoPolicy<String>` instance and wrap it in `Box::new()`.
        * Create a `Cache<String, String>` with a small capacity (e.g., 3) using the `FifoPolicy`.
        * Insert several items into the cache, exceeding its capacity to observe FIFO eviction.
        * Perform some `get` operations.
        * Print the state of the cache (e.g., its contents, and what happened during eviction).
        * Repeat the process with an `LruPolicy<String>`, observing the different eviction behavior when items are accessed.

**Conceptual Questions (to consider as you code, and answer afterwards):**

1.  **Generics (`K`, `V`):**
    * How did defining your `Cache` struct and `EvictionPolicy` trait with generic parameters (`K`, `V`) enable code reusability?
    * What are the specific trait bounds you had to add for `K` and `V` in `Cache` and `EvictionPolicy` (e.g., `Debug`, `Clone`, `Eq`, `Hash`), and why were they necessary?

2.  **Traits and Trait Objects (`Box<dyn EvictionPolicy>`):**
    * Explain why using `Box<dyn EvictionPolicy<K, V>>` for the `policy` field in your `Cache` struct was crucial for this design, compared to using a generic type parameter with trait bounds (e.g., `P: EvictionPolicy<K, V>`).
    * What are the key differences and trade-offs between static dispatch (generics with trait bounds) and dynamic dispatch (trait objects) that this challenge highlights? (Think about compile-time vs. runtime flexibility, performance, and memory layout).

3.  **Ownership and Lifetimes (A subtle aspect):**
    * The `EvictionPolicy::on_insert` method takes `K` by value (`on_insert(&mut self, key: K)`). Explain why this signature requires you to potentially `clone` the `key` when inserting into the `Cache`'s `HashMap` *and* calling `policy.on_insert`.
    * If you wanted to design a cache that stores *references* (e.g., `Cache<'a, &'a str, &'a str>`), where would explicit lifetime annotations (`'a`) be necessary in your `Cache` and `EvictionPolicy` trait/implementations? (You don't need to implement this, just explain where lifetimes would appear).

This challenge will solidify your understanding of how generics, traits, and especially trait objects work together to create flexible and powerful abstractions in Rust. Take your time, break it down, and enjoy the process!