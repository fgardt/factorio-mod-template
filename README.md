[![Release](https://github.com/fgardt/factorio-mod-template/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/fgardt/factorio-mod-template/actions/workflows/release.yml)
<!--                           ^======[REPLACE THIS]======^                                                                          ^======[REPLACE THIS]======^  -->

# factorio-mod-template

A small Factorio Mod template which also contains GitHub Actions for automatic changelog generation, packaging and releasing to the [Factorio Mod Portal](https://mods.factorio.com)

# How it works

This template uses [semantic-release](https://github.com/semantic-release/semantic-release) to automate the changelog generation aswell as packaging and releasing of the mod. \
To achieve this it analyzes your commit messages to figure out what the new version should be and what to put into the changelog.
Packaging and releasing to the factorio mod portal is done with [this plugin](https://github.com/fgardt/semantic-release-factorio). \
Additionally the GitHub Action will also create a release in your repository on GitHub itself.

Once you push new commits to the main branch the release action will trigger. \
First it will analyze all commits since the last release (determined from the last tag) to figure out if a new version should be released and what version it should be. \
To make this possible you need to follow a commit message convention. The default convention this template uses is [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) with the following types:

| Commit type                 | Changelog section |
| --------------------------- | ----------------- |
| `feat` or `feature`         | `Features`        |
| `fix`                       | `Bugfixes`        |
| `perf` or `performance`     | `Optimizations`   |
| `compat` or `compatibility` | `Compatibility`   |
| `balance`                   | `Balancing`       |
| `graphics`                  | `Graphics`        |
| `sound`                     | `Sounds`          |
| `gui`                       | `Gui`             |
| `info`                      | `Info`            |
| `locale`                    | `Locale`          |
| `translate`                 | `Translation`     |
| `control`                   | `Control`         |
| `other`                     | `Changes`         |

Because a push to the main branch triggers the release action it is recommended to work on a separate branch until your work is done and then merge that branch into main to release it. \
_Or you just work locally and if you want to release you push your changes to main, up to you how you want to do it ;)_

# How to use

## Repository setup

Click the `Use this template` button and create your own repository.

Once you have your new repository you need to add a Factorio token as a GitHub Actions secret so that the mod releasing can work. \
To get the token go to [Factorio's website](https://factorio.com/login) and login with your account. \
Then you need to go to your [profile](https://factorio.com/profile) and generate a new API key. \
The API key needs `Upload Mods`, `Publish Mods` and `Edit Mods` permissions. Copy the generated key.

Now you need to go to your repository settings > `Secrets and variables` > `Actions` and add a new Repository secret called `FACTORIO_TOKEN` with your copied key as the secret.

## Mod setup

- Swap out the [`LICENSE`](LICENSE) to your own liking _**(especially change out my name for yours)**_
- Populate the [`info.json`](info.json) file with correct values _(the `version` field gets updated automatically)_
- Add the corresponding text into [`locale.cfg`](locale/en/locale.cfg)
- Add a `thumbnail.png` to the root of the repository _([ideally 144x144px](https://wiki.factorio.com/Tutorial:Mod_structure#Files))_

# Misc

## How the packaging works

The [`semantic-release-factorio` plugin](https://github.com/fgardt/semantic-release-factorio) uses the `git archive` command to package the mod. \
That way you can specify what folders / files to exclude from your packaged mod by specifying them in [`.gitattributes`](.gitattributes).

If you want to locally test packaging of your mod you can run the following command:
```sh
git archive --format zip --prefix [YOUR-MOD-NAME]/ --worktree-attributes --output [YOUR-MOD-NAME]_[VERSION].zip HEAD
```

## Changing the commit message convention

If you want to change the commit message convention you can do so by changing the 2 `preset` fields in the [`.releaserc.json`](.releaserc.json) file. \
Possible presets are: [`angular`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-angular), [`atom`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-atom), [`codemirror`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-codemirror), [`ember`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-ember), [`eslint`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-eslint), [`express`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-express), [`jquery`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-jquery), [`jshint`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-jshint), [`conventionalcommits`](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-conventionalcommits).

Additionally you also need to modify the worflow file [`.github/workflows/release.yml`](.github/workflows/release.yml) to use the package that corresponds to your chosen preset. \
Replace `conventional-changelog-conventionalcommits` with `conventional-changelog-[YOUR PRESET]` accordingly.

## Need help?

Checkout the [official Factorio Discord](https://discord.gg/factorio) and check the pins in the `#mod-making` channel. \
There is also the [Lua API documentation](https://lua-api.factorio.com/latest/) and the [modding section in the wiki](https://wiki.factorio.com/Modding).
