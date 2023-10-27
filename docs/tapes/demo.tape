# SPDX-FileCopyrightText: 2023 Christina Sørensen
# SPDX-FileContributor: Christina Sørensen
#
# SPDX-License-Identifier: AGPL-3.0-only

Output docs/images/demo.gif

Set Shell fish

# Hide
# 
# Type "function fish_prompt -d 'Write out the prompt'"
# Enter
# Type "printf '> '"
# Enter
# Type "end"
# Enter
# 
# Show

Set FontFamily "FantasqueSansM Nerd Font Mono"

Set FontSize 20

Type "# We have already ran the rime server in the background. Let's try to connect to it to get the latest released version of fortune-kind from github."

Sleep 4s

Enter

Type "nix"

Sleep 500ms

Type " run '"

Sleep 500ms

Type "http://localhost:3000/v1/"

Sleep 500ms

Type "github"

Sleep 500ms

Type "/cafkafk"

Sleep 500ms

Type "/fortune-kind.tar.gz"

Sleep 500ms

Type "' --refresh"

Sleep 500ms

Sleep 3s

Enter

Type "# Of course, we might prefer the bleeding edge instead... Let's get the main branch."

Sleep 4s

Enter

Type "nix run 'http://localhost:3000/v1/github/cafkafk/fortune-kind/branch/main.tar.gz' --refresh"

Enter

Sleep 4s

Type "# We can also select a specific version if we'd prefer that. Let's pin to v0.1.3 and see what happens."

Sleep 4s

Enter

Type "nix run 'http://localhost:3000/v1/github/cafkafk/fortune-kind/version/v0.1.3.tar.gz' --refresh"

Enter

Sleep 4s
# Type "nix run 'http://localhost:3000/v1/github/cafkafk/fortune-kind.tar.gz' --refresh"


Sleep 5s
