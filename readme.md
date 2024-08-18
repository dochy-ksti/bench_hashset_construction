Basically, copy to Vec and convert it to Hashmap/set.
Reallocation is quite ineffective for hash collections.

If many items are duplicated, just create empty hashset and
insert items.

Basically, if there's no duplication, it takes two times 
construction time when it's created from empty Hashset
than Vec. If you can accept the cost, you don't need to copy.

When the half of the items are duplicated,
indirect construction(set the capacity of the Hashset the size 
of the Vec and copy items to the Hashset to prevent reallocation)
is not so effective, but still better than direct
construction(create empty Hashset and insert items).

```Rust
running 1 test
string
indirect 145.02ms
indirect 133.34ms
indirect 135.08ms
direct 294.48ms
direct 328.14ms
direct 291.43ms
copy and indirect 132.89ms
copy and indirect 132.89ms
copy and indirect 131.00ms
copy and direct 272.69ms
copy and direct 300.89ms
copy and direct 266.57ms
copy btree and indirect 847.33ms
copy btree and indirect 834.68ms
copy btree and indirect 851.92ms
copy, sort, dedup and indirect 444.50ms
copy, sort, dedup and indirect 456.60ms
copy, sort, dedup and indirect 441.08ms
duplicated string
indirect 191.07ms
indirect 195.59ms
indirect 191.42ms
direct 214.64ms
direct 216.12ms
direct 206.55ms
copy and indirect 199.60ms
copy and indirect 237.89ms
copy and indirect 200.16ms
copy and direct 235.80ms
copy and direct 236.79ms
copy and direct 252.44ms
copy btree and indirect 527.14ms
copy btree and indirect 557.37ms
copy btree and indirect 526.00ms
copy, sort, dedup and indirect 404.99ms
copy, sort, dedup and indirect 401.06ms
copy, sort, dedup and indirect 417.89ms
usize
indirect 55.54ms
indirect 43.47ms
indirect 43.44ms
direct 78.18ms
direct 77.52ms
direct 76.13ms
copy and indirect 49.52ms
copy and indirect 48.30ms
copy and indirect 50.65ms
copy btree and indirect 118.81ms
copy btree and indirect 114.46ms
copy btree and indirect 117.29ms
```