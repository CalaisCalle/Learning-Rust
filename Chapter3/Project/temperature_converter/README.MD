# Notes

 - Apparently "−459.67" isn't "minus 459.67", but "hyphen 459.67".
 They look too similar and it took me a minute to work out.
 - I'm shadowing "f" within the loop. I think this is (mostly) fine, but I should try and avoid this in bigger projects.
 - I've noticed that Rust won't try and convert literal types in operations. E.g. 5.0 / 2 won't compile, because 5.0 is a float and 2 is an int. 