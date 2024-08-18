Basically, copy to Vec and convert it to Hashmap/set.
Reallocation is quite ineffective for hash collections.

If many items are duplicated, just create empty hashset and
insert items.

Basically, if there's no duplication, it takes two times 
construction time when it's created from empty Hashset
than Vec. If you can accept the cost, you don't need to copy.

string
indirect 210.16ms
indirect 190.13ms
indirect 190.74ms
direct 423.37ms
direct 344.71ms
direct 341.09ms
copy and indirect 259.39ms
copy and indirect 263.87ms
copy and indirect 258.91ms
copy and direct 415.27ms
copy and direct 418.81ms
copy and direct 412.11ms
copy btree and indirect 831.95ms
copy btree and indirect 835.48ms
copy btree and indirect 825.23ms

When the half of the items are duplicated,
indirect construction(set the capacity of the Hashset the size 
of the Vec and copy items to the Hashset to prevent reallocation)
is not so effective, but still better than direct
construction(create empty Hashset and insert items).

duplicated string
indirect 246.51ms
indirect 241.37ms
indirect 244.50ms
direct 270.68ms
direct 267.45ms
direct 268.74ms
copy and indirect 316.07ms
copy and indirect 312.17ms
copy and indirect 313.08ms
copy and direct 338.04ms
copy and direct 344.18ms
copy and direct 337.20ms
copy btree and indirect 687.59ms
copy btree and indirect 711.10ms
copy btree and indirect 683.17ms

usize
indirect 69.10ms
indirect 72.78ms
indirect 72.18ms
direct 102.32ms
direct 99.43ms
direct 103.86ms
copy and indirect 85.73ms
copy and indirect 85.45ms
copy and indirect 84.63ms
copy btree and indirect 164.42ms
copy btree and indirect 212.72ms
copy btree and indirect 158.25ms