# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v2.7.0](https://codeberg.org/PurpleBooth/git-moves-together/compare/b5bfc29644d554cf0d29f06fa7eae2d622b60eae..v2.7.0) - 2025-05-11
#### Features
- add docker-bake.hcl for multi-platform builds - ([09995b0](https://codeberg.org/PurpleBooth/git-moves-together/commit/09995b07373a916def2245bcd0f445f1cfff6410)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust:alpine docker digest to d6e876c - ([be95dcb](https://codeberg.org/PurpleBooth/git-moves-together/commit/be95dcb913cf3a13bee03c02205d07ba69623592)) - renovate[bot]
- **(deps)** update rust:alpine docker digest to d57abe5 - ([203a47f](https://codeberg.org/PurpleBooth/git-moves-together/commit/203a47f53ad04c7efa5b056b092c18245491a46b)) - renovate[bot]
- **(deps)** update rust:alpine docker digest to e4ab5bd - ([b5bfc29](https://codeberg.org/PurpleBooth/git-moves-together/commit/b5bfc29644d554cf0d29f06fa7eae2d622b60eae)) - renovate[bot]
- Update repository URLs from GitHub to Codeberg - ([bcf381f](https://codeberg.org/PurpleBooth/git-moves-together/commit/bcf381f8fb03eaf8719447beaef69c0f252b3c66)) - Billie Thompson
- Update Dockerfile to use latest rust:alpine image - ([92b1735](https://codeberg.org/PurpleBooth/git-moves-together/commit/92b1735be22215d8b603ef298e3e2150ef1d26e4)) - Billie Thompson
- update dependencies and refactor code - ([e9a1753](https://codeberg.org/PurpleBooth/git-moves-together/commit/e9a175315fb0a1496570c7a6cab506d20c0ab0e6)) - Billie Thompson
#### Refactoring
- Restore original test block in homebrew formula - ([956c55b](https://codeberg.org/PurpleBooth/git-moves-together/commit/956c55be76e94c0604362e7469222d2286ed499c)) - Billie Thompson (aider)
- update formula template to match whatismyip style - ([eaea71c](https://codeberg.org/PurpleBooth/git-moves-together/commit/eaea71cc14d1afb52f474953610c17807e8a93ff)) - Billie Thompson (aider)

- - -

## [v2.6.3](https://codeberg.org/PurpleBooth/git-moves-together/compare/33be24fc3569c08f4bb5f0301b9bb4698e35d684..v2.6.3) - 2024-08-31
#### Bug Fixes
- **(deps)** update rust crate tokio to 1.40.0 - ([4539f57](https://codeberg.org/PurpleBooth/git-moves-together/commit/4539f570f876fcb8f3ddb4658d96856075865068)) - renovate[bot]
#### Continuous Integration
- Simplify push - ([1f8099d](https://codeberg.org/PurpleBooth/git-moves-together/commit/1f8099da1f9f31ef1ca21ada45056a9275c53b4b)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin dependencies - ([4adc75a](https://codeberg.org/PurpleBooth/git-moves-together/commit/4adc75a5873808130ec156fdcc472a254ca45bf8)) - renovate[bot]
#### Refactoring
- Rewrie docker file - ([c6f01df](https://codeberg.org/PurpleBooth/git-moves-together/commit/c6f01df4af48c595a4bf91af82127d72def044fe)) - Billie Thompson
- Cross compile to reduce build time - ([33be24f](https://codeberg.org/PurpleBooth/git-moves-together/commit/33be24fc3569c08f4bb5f0301b9bb4698e35d684)) - Billie Thompson

- - -

## [v2.6.2](https://codeberg.org/PurpleBooth/git-moves-together/compare/4a426fa0581974281a57f067a4641c73ab9c4613..v2.6.2) - 2024-08-30
#### Bug Fixes
- test deploy - ([4a426fa](https://codeberg.org/PurpleBooth/git-moves-together/commit/4a426fa0581974281a57f067a4641c73ab9c4613)) - Billie Thompson

- - -

## [v2.6.1](https://codeberg.org/PurpleBooth/git-moves-together/compare/2d4cabb1777fc973e912094085751f170212b6dd..v2.6.1) - 2024-08-24
#### Bug Fixes
- Bump versions - ([a33d33f](https://codeberg.org/PurpleBooth/git-moves-together/commit/a33d33f4587d7e4928bd4284f2bf7854876a312a)) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** pin rust docker tag to 1f5aff5 - ([2d4cabb](https://codeberg.org/PurpleBooth/git-moves-together/commit/2d4cabb1777fc973e912094085751f170212b6dd)) - renovate[bot]

- - -

## [v2.6.0](https://codeberg.org/PurpleBooth/git-moves-together/compare/ed6f1a981272cd3bbf73199eed1dbee1213636f4..v2.6.0) - 2024-08-24
#### Bug Fixes
- Disable clap debug - ([b151981](https://codeberg.org/PurpleBooth/git-moves-together/commit/b151981356bbada2976b2a9d80bf77a202a74060)) - Billie Thompson
- Switch to alpine based docker image for simpler musl builds - ([0b8712c](https://codeberg.org/PurpleBooth/git-moves-together/commit/0b8712cbb181f8a5fc983914c90b2b09af8fdbca)) - Billie Thompson
#### Build system
- Add static ssl dep - ([7e002db](https://codeberg.org/PurpleBooth/git-moves-together/commit/7e002db22a63f9acc6d6956df994bc5beaa11a71)) - Billie Thompson
- Add a user to run binary with - ([1b11409](https://codeberg.org/PurpleBooth/git-moves-together/commit/1b11409691789bf8c9aa962c637866a51502ee29)) - Billie Thompson
- Set CC and CXX variables - ([df9db7d](https://codeberg.org/PurpleBooth/git-moves-together/commit/df9db7d184cd238b96e804c7473c7ba50fd4a1b5)) - Billie Thompson
#### Continuous Integration
- Delete mergify file that is unused - ([1a5e0f4](https://codeberg.org/PurpleBooth/git-moves-together/commit/1a5e0f4ec6e8ead7b2b38069a69563d9ecdb7867)) - Billie Thompson
- Delete versio file that is unused - ([e742dad](https://codeberg.org/PurpleBooth/git-moves-together/commit/e742dad529221e3b4eeee972adfc546373f06623)) - Billie Thompson
- Delete dependabot file that is unused - ([ed6f1a9](https://codeberg.org/PurpleBooth/git-moves-together/commit/ed6f1a981272cd3bbf73199eed1dbee1213636f4)) - Billie Thompson
#### Documentation
- Remove "only x86", as we build for aarch - ([4f0556f](https://codeberg.org/PurpleBooth/git-moves-together/commit/4f0556f69d138609dd1d3922234d7a55d7fb8095)) - Billie Thompson
#### Features
- Offer a vendored version of the bin - ([6eb7726](https://codeberg.org/PurpleBooth/git-moves-together/commit/6eb77268b72cc08a8aa22b35b7dcd315903fe694)) - Billie Thompson
- Add docker image - ([3ac8a42](https://codeberg.org/PurpleBooth/git-moves-together/commit/3ac8a42d471559f7a864c8e58753ec68f3334bf3)) - Billie Thompson

- - -

## [v2.5.71](https://codeberg.org/PurpleBooth/git-moves-together/compare/946d4ac034a53ba5ecc8e252ac141999853043ac..v2.5.71) - 2024-08-19
#### Bug Fixes
- **(deps)** bump tokio from 1.39.2 to 1.39.3 - ([946d4ac](https://codeberg.org/PurpleBooth/git-moves-together/commit/946d4ac034a53ba5ecc8e252ac141999853043ac)) - dependabot[bot]

- - -

## [v2.5.70](https://codeberg.org/PurpleBooth/git-moves-together/compare/d9d99f5ceb94cd5106c4cdbced214283dba458d9..v2.5.70) - 2024-08-16
#### Bug Fixes
- **(deps)** bump clap from 4.5.15 to 4.5.16 - ([d9d99f5](https://codeberg.org/PurpleBooth/git-moves-together/commit/d9d99f5ceb94cd5106c4cdbced214283dba458d9)) - dependabot[bot]

- - -

## [v2.5.69](https://codeberg.org/PurpleBooth/git-moves-together/compare/84fe2efbd5185313e670b4f752aaf1344885d4e6..v2.5.69) - 2024-08-12
#### Bug Fixes
- **(deps)** bump clap from 4.5.14 to 4.5.15 - ([84fe2ef](https://codeberg.org/PurpleBooth/git-moves-together/commit/84fe2efbd5185313e670b4f752aaf1344885d4e6)) - dependabot[bot]

- - -

## [v2.5.68](https://codeberg.org/PurpleBooth/git-moves-together/compare/405b996d60b31de4b4d58c1ffc4f9ddb416af758..v2.5.68) - 2024-08-09
#### Bug Fixes
- **(deps)** bump clap from 4.5.13 to 4.5.14 - ([405b996](https://codeberg.org/PurpleBooth/git-moves-together/commit/405b996d60b31de4b4d58c1ffc4f9ddb416af758)) - dependabot[bot]

- - -

## [v2.5.67](https://codeberg.org/PurpleBooth/git-moves-together/compare/69863021e6e9c0c55fe2f7aeec79f3c5daf8e8b2..v2.5.67) - 2024-08-07
#### Bug Fixes
- **(deps)** bump tempfile from 3.11.0 to 3.12.0 - ([6986302](https://codeberg.org/PurpleBooth/git-moves-together/commit/69863021e6e9c0c55fe2f7aeec79f3c5daf8e8b2)) - dependabot[bot]

- - -

## [v2.5.66](https://codeberg.org/PurpleBooth/git-moves-together/compare/f0ab32e15825f8faf60e9ec91293cc2a6e31c66f..v2.5.66) - 2024-08-05
#### Bug Fixes
- **(deps)** bump tempfile from 3.10.1 to 3.11.0 - ([f0ab32e](https://codeberg.org/PurpleBooth/git-moves-together/commit/f0ab32e15825f8faf60e9ec91293cc2a6e31c66f)) - dependabot[bot]

- - -

## [v2.5.65](https://codeberg.org/PurpleBooth/git-moves-together/compare/66036cfc9e7eaead43fe8ff90e55a5850c898447..v2.5.65) - 2024-08-02
#### Bug Fixes
- Update openssl binding - ([66036cf](https://codeberg.org/PurpleBooth/git-moves-together/commit/66036cfc9e7eaead43fe8ff90e55a5850c898447)) - Billie Thompson

- - -

## [v2.5.64](https://codeberg.org/PurpleBooth/git-moves-together/compare/4948adfbbc759a78cb3a78aed0a3771cc3d72ec7..v2.5.64) - 2024-08-02
#### Bug Fixes
- Update if to the binary release - ([3609176](https://codeberg.org/PurpleBooth/git-moves-together/commit/3609176044c5670749be03ea83775e7047801b3e)) - Billie Thompson
#### Build system
- Correct server - ([72a0670](https://codeberg.org/PurpleBooth/git-moves-together/commit/72a0670a187763a4efb5f3ac73b94a5fcfd70973)) - Billie Thompson
#### Continuous Integration
- Re-add commit checks - ([10e0519](https://codeberg.org/PurpleBooth/git-moves-together/commit/10e05197abcd175c6aa7d69b7b85105cfe137281)) - Billie Thompson
- use cog - ([4948adf](https://codeberg.org/PurpleBooth/git-moves-together/commit/4948adfbbc759a78cb3a78aed0a3771cc3d72ec7)) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).