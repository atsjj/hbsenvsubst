# hbsenvsubst

Substitutes the values of environment variables, but with handlebars.

## About

Basically the idea of `hbsenvsubst` is to work similar to `envsubst`, but instead of using
shell format, it uses handlebars.

Please be aware that at this time, `hbsenvsubst` should be considered a prototype that was quickly
rushed to play with this idea, so your mileage may vary.

## Usage

```shell
hbsenvsubst < some-file-in > some-file-out
```

### Example

An example handlebars input like this:

```shell
#!/bin/bash

echo "hello {{env.USER}}"
```

should yield `hello foo`, given that the `USER` environment variable is "foo".

## Features

There are currently three objects accessible in handlebars:

* `env` - environment variables
* `mem`
* * `free` - free system memory
* * `total` - total system memory
* * `used` - used system memory
* `cpu`
* * `logical` - number of logical cpus
* * `physical` - number of physical cpus

In addition to this, there are some additional helpers:

* `add` - add
* * `{{add 2 2}}` yields `4`
* `div` - divide
* * `{{div 4 2}}` yields `2`
* `mod` - modulo
* * `{{mod 4 3}}` yields `1`
* `mul` - multiply
* * `{{mul 3 3}}` yields `9`
* `sub` - subtract
* * `{{sub 8 5}}` yields `3`

The [rust-handlebars](https://github.com/sunng87/handlebars-rust) crate also includes some built-in
helpers:

* `if`
* `unless`
* `log`
* `each`
* `with`
* `eq`
* `ne`
* `gt`
* `gte`
* `lt`
* `lte`
* `and`
* `or`
* `not`

## License

MIT License
