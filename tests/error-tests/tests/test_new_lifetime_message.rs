// 1.21 added new support for messages with multiple spans.
/*BEGIN*/fn foo(x: &mut Vec<&u32>, y: &u32) {
//                          ^ERR(>=1.63.0-beta) let's call the lifetime of this reference `'1`
//                                    ^ERR(>=1.63.0-beta) let's call the lifetime of this reference `'2`
//                          ^^^^ERR(>=1.21.0,<1.63.0-beta)
//                                    ^^^^ERR(>=1.21.0,<1.63.0-beta) these two types
//                                    ^^^^MSG(>=1.21.0,<1.63.0-beta) See Primary: ↓:15
//             |HELP(>=1.58.0-beta) consider introducing
//             |HELP(>=1.58.0-beta) /Accept Replacement:.*<'a>/
//                           |HELP(>=1.58.0-beta) consider introducing
//                           |HELP(>=1.58.0-beta) /Accept Replacement:.*'a/
//                                     |HELP(>=1.58.0-beta) consider introducing
//                                     |HELP(>=1.58.0-beta) /Accept Replacement:.*'a/
//                                    ^MSG(>=1.63.0-beta) See Primary: ↓:15
  x.push(y);
//       ^ERR(>=1.21.0,<1.63.0-beta) lifetime mismatch
//       ^ERR(>=1.21.0,<1.63.0-beta) ...but data from
//       ^ERR(<1.21.0,<1.63.0-beta) lifetime of reference outlives
//       ^NOTE(>=1.58.0-beta,<1.63.0-beta) each elided lifetime
//       ^MSG(>=1.21.0,<1.63.0-beta) See Also: ↑:2
//^^^^^^^^^ERR(>=1.63.0-beta) lifetime may not live long enough
//^^^^^^^^^ERR(>=1.63.0-beta) argument requires that
//^^^^^^^^^MSG(>=1.63.0-beta) See Also: ↑:2
//^^^^^^^^^NOTE(>=1.86.0-beta) requirement occurs because
//^^^^^^^^^NOTE(>=1.86.0-beta) mutable references are invariant
//^^^^^^^^^HELP(>=1.86.0-beta) see
}/*END*/
// ~NOTE(<1.21.0,<1.63.0-beta) ...the reference is valid
// ~NOTE(<1.21.0,<1.63.0-beta) ...but the borrowed content
// ~HELP(<1.16.0,<1.63.0-beta) consider using an explicit lifetime
