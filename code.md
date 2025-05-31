# ex: "Hello, world!"
    H: 0048
    e: 0065
    l-l: 006c
    o: 006f
    ,: 002c
    <space>: 0020
    w: 0077
    o: 006f
    r: 0072
    l: 006c
    d: 0064
    !: 0021

# ex(bytes, separator: \) 0048\0065\006c\006c\006f\002c\0020\0077\006f\0072\006c\0064\0021

# Logic behind the code
  All this code just rearranges bytes so it can apply to any data basically.
  If we apply the logic mentioned in `readme.md` on the example above what should happen is:
  `SBE` flips everything and splits it into groups of 5 and we are going to apply logic on the first one: 0021\0064\006c\0072\006f
  then it chooses a replace char (randomly using `rand`), suppose it's 0072 (index `3`): 0021\0064\006c\006f\0072\0003
  replace char goes to the end and index gets added

