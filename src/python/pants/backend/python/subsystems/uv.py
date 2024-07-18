# Copyright 2022 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

from __future__ import annotations

from pants.backend.python.subsystems.python_tool_base import PythonToolBase
from pants.backend.python.target_types import EntryPoint
from pants.engine.rules import collect_rules
from pants.option.option_types import ArgsListOption


class UvSubsystem(PythonToolBase):
    options_scope = "uv"
    name = options_scope
    help_short = "uv (https://github.com/astral-sh/uv)"

    default_main = EntryPoint("uv")
    default_interpreter_constraints = ["CPython>=3.8,<4"]
    default_requirements = ["uv>=0.2"]

    register_interpreter_constraints = True

    default_lockfile_resource = ("pants.backend.python.subsystems", "uv.lock")

    args = ArgsListOption(example="--verbose --no-cache")




def rules():
    return collect_rules()
