#!/usr/bin/env python3
import copy
import json
import subprocess

SPECS = {
    "niti-v1": {
        "cpu": "atmega2560",
    },
    "atmega164pa": {
        "cpu": "atmega164pa",
    },
    "atmega168": {
        "cpu": "atmega168",
    },
    
    "atmega328": {
        "cpu": "atmega328",
    },
    "atmega328p": {
        "cpu": "atmega328p",
    },
    "atmega2560": {
        "cpu": "atmega2560",
    },
    
}

COMMON = {
    # needed because we currently rely on avr-libc
    "no-default-libraries": False,
    # 8-bit operations on AVR are atomic
    # LLVM also supports 16-bit atomics by disabling interrupts
    # see also https://github.com/rust-lang/rust/pull/114495
    "max-atomic-width": 16,
}


def main():
    rustc_version = subprocess.run(
        ["rustc", "--version"],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout.decode()

    if "nightly" not in rustc_version:
        raise Exception("You need nightly rustc!")

    upstream_spec_string = subprocess.run(
        [
            "rustc",
            "--print",
            "target-spec-json",
            "-Z",
            "unstable-options",
            "--target",
            "avr-unknown-gnu-atmega328",
        ],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout

    upstream_spec = json.loads(upstream_spec_string)

    # our targets are of course not built into rustc
    del upstream_spec["is-builtin"]

    for mcu, settings in SPECS.items():
        spec = copy.deepcopy(upstream_spec)
        spec.update(COMMON)
        spec.update(settings)

        for pre_link_args in spec["pre-link-args"].values():
            pre_link_args[0] = f"-mmcu={settings['cpu']}"
            pre_link_args.append("-Wl,--as-needed,--print-memory-usage")

        with open(f"avr-specs/avr-{mcu}.json", "w") as f:
            json.dump(spec, f, sort_keys=True, indent=2)
            f.write("\n")


if __name__ == "__main__":
    main()
