# Notes about all of this.

## yaml stuff

* The docs, once again, fail to quote various things. In short it's
  better to quote than not quote.

* All "tag" values must be defined in `tags.yaml`, which you include
  BEFORE any file that uses them.

* You _are_ going to need to install `glean_parser` and make it accessible somewhere.
  This is a python script, so probably best to `python -mvenv venv; venv/bin/pip3 install -r requirements.txt`
  (I fully expect Taddes to slap me for being old and not creating a pyproject.toml file.)

## command

### create the rust stuff

You _can_ try to compile the rust file directly, using the following command:
`glean_parser translate -o src -f rust tags.yaml metrics.yaml`

`build.rs` contains basically the same thing, and requires the python builder. Kind of up to you on if you want it
automagically run by `cargo build` or manually with the above command.

This will auto-generate a `{-o output directory}/glean_metrics.rs` file that notes that it should NOT be checked in. Going to argue that it _should_ be checked in since rust is not exactly good with "magically generated files".

The file creates a mod that matches the `metrics.yaml:Category` that contains various pub static that match the `metrics.yaml:Category:Name` fields.
_*NOTE*_: the `event` type is currently broken in glean_parser 15.0.0. See https://bugzilla.mozilla.org/show_bug.cgi?id=1915744

* I am NOT a fan of pointing directly to the glean repo for `glean-build`. Should we vendor that?

