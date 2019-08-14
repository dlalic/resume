# Resume

[![Build Status](https://travis-ci.org/dlalic/resume.svg?branch=master)](https://travis-ci.org/dlalic/resume)

This is not a serious project, more of a `Rust` exercise. The goal is to have a resume structured in a human readable `yaml` and use it to populate `TeX`, LinkedIn, Xing and others. A similar project is [`pyresume`](https://github.com/waynr/pyresume).

Didn't use [`tectonic`](https://github.com/tectonic-typesetting/tectonic/) due to external dependencies, render the output with local `XeLaTeX` due to custom font usage.

## Current stage

`cargo run` outputs a `.tex` file that renders to the below pdf based on the data in [`examples/resume.yaml`](examples/resume.yaml).

![resume](examples/resume.jpg)