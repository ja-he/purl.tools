use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PurlType {
    Alpm,
    Android,
    Apache,
    Apk,
    Atom,
    Bitbucket,
    Bitnami,
    Bower,
    Brew,
    Buildroot,
    Cargo,
    Carthage,
    Chef,
    Chocolatey,
    Clojars,
    Cocoapods,
    Composer,
    Conan,
    Conda,
    Coreos,
    Cpan,
    Cran,
    Crystal,
    Ctan,
    Deb,
    Docker,
    Drupal,
    Dtype,
    Dub,
    Ebuild,
    Eclipse,
    Elm,
    Gem,
    Generic,
    Gitea,
    Github,
    Gitlab,
    Golang,
    Gradle,
    Guix,
    Hackage,
    Haxe,
    Helm,
    Hex,
    Huggingface,
    Julia,
    Lua,
    Maven,
    Melpa,
    Meteor,
    Mlflow,
    Nim,
    Nix,
    Npm,
    Nuget,
    Oci,
    Opam,
    Openwrt,
    Osgi,
    P2,
    Pear,
    Pecl,
    Perl6,
    Platformio,
    Pub,
    Puppet,
    Pypi,
    Qpkg,
    Rpm,
    Sourceforge,
    Sublime,
    Swid,
    Swift,
    Terraform,
    Vagrant,
    Vim,
    Wordpress,
    Yocto,
    Other(String),
}

impl PurlType {
    pub fn new(s: &str) -> PurlType {
        match s {
            "alpm" => Self::Alpm,
            "android" => Self::Android,
            "apache" => Self::Apache,
            "apk" => Self::Apk,
            "atom" => Self::Atom,
            "bitbucket" => Self::Bitbucket,
            "bitnami" => Self::Bitnami,
            "bower" => Self::Bower,
            "brew" => Self::Brew,
            "buildroot" => Self::Buildroot,
            "cargo" => Self::Cargo,
            "carthage" => Self::Carthage,
            "chef" => Self::Chef,
            "chocolatey" => Self::Chocolatey,
            "clojars" => Self::Clojars,
            "cocoapods" => Self::Cocoapods,
            "composer" => Self::Composer,
            "conan" => Self::Conan,
            "conda" => Self::Conda,
            "coreos" => Self::Coreos,
            "cpan" => Self::Cpan,
            "cran" => Self::Cran,
            "crystal" => Self::Crystal,
            "ctan" => Self::Ctan,
            "deb" => Self::Deb,
            "docker" => Self::Docker,
            "drupal" => Self::Drupal,
            "dtype" => Self::Dtype,
            "dub" => Self::Dub,
            "ebuild" => Self::Ebuild,
            "eclipse" => Self::Eclipse,
            "elm" => Self::Elm,
            "gem" => Self::Gem,
            "generic" => Self::Generic,
            "gitea" => Self::Gitea,
            "github" => Self::Github,
            "gitlab" => Self::Gitlab,
            "golang" => Self::Golang,
            "gradle" => Self::Gradle,
            "guix" => Self::Guix,
            "hackage" => Self::Hackage,
            "haxe" => Self::Haxe,
            "helm" => Self::Helm,
            "hex" => Self::Hex,
            "huggingface" => Self::Huggingface,
            "julia" => Self::Julia,
            "lua" => Self::Lua,
            "maven" => Self::Maven,
            "melpa" => Self::Melpa,
            "meteor" => Self::Meteor,
            "mlflow" => Self::Mlflow,
            "nim" => Self::Nim,
            "nix" => Self::Nix,
            "npm" => Self::Npm,
            "nuget" => Self::Nuget,
            "oci" => Self::Oci,
            "opam" => Self::Opam,
            "openwrt" => Self::Openwrt,
            "osgi" => Self::Osgi,
            "p2" => Self::P2,
            "pear" => Self::Pear,
            "pecl" => Self::Pecl,
            "perl6" => Self::Perl6,
            "platformio" => Self::Platformio,
            "pub" => Self::Pub,
            "puppet" => Self::Puppet,
            "pypi" => Self::Pypi,
            "qpkg" => Self::Qpkg,
            "rpm" => Self::Rpm,
            "sourceforge" => Self::Sourceforge,
            "sublime" => Self::Sublime,
            "swid" => Self::Swid,
            "swift" => Self::Swift,
            "terraform" => Self::Terraform,
            "vagrant" => Self::Vagrant,
            "vim" => Self::Vim,
            "wordpress" => Self::Wordpress,
            "yocto" => Self::Yocto,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn status(&self) -> PurlTypeStatus {
        match self {
            Self::Alpm => PurlTypeStatus::WellKnown,
            Self::Android => PurlTypeStatus::Proposed,
            Self::Apache => PurlTypeStatus::Proposed,
            Self::Apk => PurlTypeStatus::WellKnown,
            Self::Atom => PurlTypeStatus::Proposed,
            Self::Bitbucket => PurlTypeStatus::WellKnown,
            Self::Bitnami => PurlTypeStatus::WellKnown,
            Self::Bower => PurlTypeStatus::Proposed,
            Self::Brew => PurlTypeStatus::Proposed,
            Self::Buildroot => PurlTypeStatus::Proposed,
            Self::Cargo => PurlTypeStatus::WellKnown,
            Self::Carthage => PurlTypeStatus::Proposed,
            Self::Chef => PurlTypeStatus::Proposed,
            Self::Chocolatey => PurlTypeStatus::Proposed,
            Self::Clojars => PurlTypeStatus::Proposed,
            Self::Cocoapods => PurlTypeStatus::WellKnown,
            Self::Composer => PurlTypeStatus::WellKnown,
            Self::Conan => PurlTypeStatus::WellKnown,
            Self::Conda => PurlTypeStatus::WellKnown,
            Self::Coreos => PurlTypeStatus::Proposed,
            Self::Cpan => PurlTypeStatus::Proposed,
            Self::Cran => PurlTypeStatus::WellKnown,
            Self::Crystal => PurlTypeStatus::Proposed,
            Self::Ctan => PurlTypeStatus::Proposed,
            Self::Deb => PurlTypeStatus::WellKnown,
            Self::Docker => PurlTypeStatus::WellKnown,
            Self::Drupal => PurlTypeStatus::Proposed,
            Self::Dtype => PurlTypeStatus::Proposed,
            Self::Dub => PurlTypeStatus::Proposed,
            Self::Ebuild => PurlTypeStatus::Proposed,
            Self::Eclipse => PurlTypeStatus::Proposed,
            Self::Elm => PurlTypeStatus::Proposed,
            Self::Gem => PurlTypeStatus::WellKnown,
            Self::Generic => PurlTypeStatus::WellKnown,
            Self::Gitea => PurlTypeStatus::Proposed,
            Self::Github => PurlTypeStatus::WellKnown,
            Self::Gitlab => PurlTypeStatus::Proposed,
            Self::Golang => PurlTypeStatus::WellKnown,
            Self::Gradle => PurlTypeStatus::Proposed,
            Self::Guix => PurlTypeStatus::Proposed,
            Self::Hackage => PurlTypeStatus::WellKnown,
            Self::Haxe => PurlTypeStatus::Proposed,
            Self::Helm => PurlTypeStatus::Proposed,
            Self::Hex => PurlTypeStatus::WellKnown,
            Self::Huggingface => PurlTypeStatus::WellKnown,
            Self::Julia => PurlTypeStatus::Proposed,
            Self::Lua => PurlTypeStatus::Proposed,
            Self::Maven => PurlTypeStatus::WellKnown,
            Self::Melpa => PurlTypeStatus::Proposed,
            Self::Meteor => PurlTypeStatus::Proposed,
            Self::Mlflow => PurlTypeStatus::WellKnown,
            Self::Nim => PurlTypeStatus::Proposed,
            Self::Nix => PurlTypeStatus::Proposed,
            Self::Npm => PurlTypeStatus::WellKnown,
            Self::Nuget => PurlTypeStatus::WellKnown,
            Self::Oci => PurlTypeStatus::WellKnown,
            Self::Opam => PurlTypeStatus::Proposed,
            Self::Openwrt => PurlTypeStatus::Proposed,
            Self::Osgi => PurlTypeStatus::Proposed,
            Self::P2 => PurlTypeStatus::Proposed,
            Self::Pear => PurlTypeStatus::Proposed,
            Self::Pecl => PurlTypeStatus::Proposed,
            Self::Perl6 => PurlTypeStatus::Proposed,
            Self::Platformio => PurlTypeStatus::Proposed,
            Self::Pub => PurlTypeStatus::WellKnown,
            Self::Puppet => PurlTypeStatus::Proposed,
            Self::Pypi => PurlTypeStatus::WellKnown,
            Self::Qpkg => PurlTypeStatus::WellKnown,
            Self::Rpm => PurlTypeStatus::WellKnown,
            Self::Sourceforge => PurlTypeStatus::Proposed,
            Self::Sublime => PurlTypeStatus::Proposed,
            Self::Swid => PurlTypeStatus::WellKnown,
            Self::Swift => PurlTypeStatus::WellKnown,
            Self::Terraform => PurlTypeStatus::Proposed,
            Self::Vagrant => PurlTypeStatus::Proposed,
            Self::Vim => PurlTypeStatus::Proposed,
            Self::Wordpress => PurlTypeStatus::Proposed,
            Self::Yocto => PurlTypeStatus::Proposed,
            Self::Other(_) => PurlTypeStatus::Other,
        }
    }
}

impl fmt::Display for PurlType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alpm => "alpm",
                Self::Android => "android",
                Self::Apache => "apache",
                Self::Apk => "apk",
                Self::Atom => "atom",
                Self::Bitbucket => "bitbucket",
                Self::Bitnami => "bitnami",
                Self::Bower => "bower",
                Self::Brew => "brew",
                Self::Buildroot => "buildroot",
                Self::Cargo => "cargo",
                Self::Carthage => "carthage",
                Self::Chef => "chef",
                Self::Chocolatey => "chocolatey",
                Self::Clojars => "clojars",
                Self::Cocoapods => "cocoapods",
                Self::Composer => "composer",
                Self::Conan => "conan",
                Self::Conda => "conda",
                Self::Coreos => "coreos",
                Self::Cpan => "cpan",
                Self::Cran => "cran",
                Self::Crystal => "crystal",
                Self::Ctan => "ctan",
                Self::Deb => "deb",
                Self::Docker => "docker",
                Self::Drupal => "drupal",
                Self::Dtype => "dtype",
                Self::Dub => "dub",
                Self::Ebuild => "ebuild",
                Self::Eclipse => "eclipse",
                Self::Elm => "elm",
                Self::Gem => "gem",
                Self::Generic => "generic",
                Self::Gitea => "gitea",
                Self::Github => "github",
                Self::Gitlab => "gitlab",
                Self::Golang => "golang",
                Self::Gradle => "gradle",
                Self::Guix => "guix",
                Self::Hackage => "hackage",
                Self::Haxe => "haxe",
                Self::Helm => "helm",
                Self::Hex => "hex",
                Self::Huggingface => "huggingface",
                Self::Julia => "julia",
                Self::Lua => "lua",
                Self::Maven => "maven",
                Self::Melpa => "melpa",
                Self::Meteor => "meteor",
                Self::Mlflow => "mlflow",
                Self::Nim => "nim",
                Self::Nix => "nix",
                Self::Npm => "npm",
                Self::Nuget => "nuget",
                Self::Oci => "oci",
                Self::Opam => "opam",
                Self::Openwrt => "openwrt",
                Self::Osgi => "osgi",
                Self::P2 => "p2",
                Self::Pear => "pear",
                Self::Pecl => "pecl",
                Self::Perl6 => "perl6",
                Self::Platformio => "platformio",
                Self::Pub => "pub",
                Self::Puppet => "puppet",
                Self::Pypi => "pypi",
                Self::Qpkg => "qpkg",
                Self::Rpm => "rpm",
                Self::Sourceforge => "sourceforge",
                Self::Sublime => "sublime",
                Self::Swid => "swid",
                Self::Swift => "swift",
                Self::Terraform => "terraform",
                Self::Vagrant => "vagrant",
                Self::Vim => "vim",
                Self::Wordpress => "wordpress",
                Self::Yocto => "yocto",
                Self::Other(s) => s,
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurlTypeStatus {
    WellKnown,
    Proposed,
    Other,
}

pub const PURL_TYPES: &[PurlType] = &[
    PurlType::Alpm,
    PurlType::Android,
    PurlType::Apache,
    PurlType::Apk,
    PurlType::Atom,
    PurlType::Bitbucket,
    PurlType::Bitnami,
    PurlType::Bower,
    PurlType::Brew,
    PurlType::Buildroot,
    PurlType::Cargo,
    PurlType::Carthage,
    PurlType::Chef,
    PurlType::Chocolatey,
    PurlType::Clojars,
    PurlType::Cocoapods,
    PurlType::Composer,
    PurlType::Conan,
    PurlType::Conda,
    PurlType::Coreos,
    PurlType::Cpan,
    PurlType::Cran,
    PurlType::Crystal,
    PurlType::Ctan,
    PurlType::Deb,
    PurlType::Docker,
    PurlType::Drupal,
    PurlType::Dtype,
    PurlType::Dub,
    PurlType::Ebuild,
    PurlType::Eclipse,
    PurlType::Elm,
    PurlType::Gem,
    PurlType::Generic,
    PurlType::Gitea,
    PurlType::Github,
    PurlType::Gitlab,
    PurlType::Golang,
    PurlType::Gradle,
    PurlType::Guix,
    PurlType::Hackage,
    PurlType::Haxe,
    PurlType::Helm,
    PurlType::Hex,
    PurlType::Huggingface,
    PurlType::Julia,
    PurlType::Lua,
    PurlType::Maven,
    PurlType::Melpa,
    PurlType::Meteor,
    PurlType::Mlflow,
    PurlType::Nim,
    PurlType::Nix,
    PurlType::Npm,
    PurlType::Nuget,
    PurlType::Oci,
    PurlType::Opam,
    PurlType::Openwrt,
    PurlType::Osgi,
    PurlType::P2,
    PurlType::Pear,
    PurlType::Pecl,
    PurlType::Perl6,
    PurlType::Platformio,
    PurlType::Pub,
    PurlType::Puppet,
    PurlType::Pypi,
    PurlType::Qpkg,
    PurlType::Rpm,
    PurlType::Sourceforge,
    PurlType::Sublime,
    PurlType::Swid,
    PurlType::Swift,
    PurlType::Terraform,
    PurlType::Vagrant,
    PurlType::Vim,
    PurlType::Wordpress,
    PurlType::Yocto,
];

pub fn parse_purl_namespace(s: &str) -> Vec<String> {
    s.split('/').map(|s| s.to_string()).collect()
}
