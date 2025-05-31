# Disclaimer
  This library should not be used for production and I give permission for anyone to edit it and add it to their project

# Usage
  `SBE::encrypt::encrypt<'a>(s: String) -> &'a [u8]`:
    First, understanding the process:
      `SBE` takes every 5 bytes and flips them, then it adds after them another 5 bytes, those bytes are responsible for determining the randomly chosen and replaced byte which is called the `replace_char` which gets put at the end and if the byte array didn't end in perfect fives it fills the rest of `meta_bytes` with `1`
    Second, what is the output:
      the output is a `refrence` which means that you need to `.clone()` it to ensure that `ownership` is under control
  `SBE::decrypt::decrypt(b: &[u8]) -> String`:
  First, understanding the process: 
    this reverses the function above by first reading the 5 `meta_bytes` before reading the 5 `bytes` themselves and puts the `replace_char` in it's original place then re-flips them to get to the original state and clears the `meta_bytes`
  Second, usage:
    the usage of the `decrypt` function is much easier sense it gives you the ownership which means you just put in the `byte_array` and everything should work

**Note**: This library should be used on top of an encryption library like Base64

**Note**: if you want deeper information about the encode process refer to `code.md`

# Reporting bugs
If you find any bugs then make sure that you send it in the issues tab/page

# Contribution
Currently I'm trying to get contributors sense the project is literally 2 functions but the idea seems cool especially for experiments, temporary security and CyberSecurity

# Credits
- KernelState
