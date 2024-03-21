hidden = true

[metadata]
title = "My attempt at managing NixOS secrets"
category = "nix"
tags = ["nix", "secrets", "nixos"]
date = 3
description = "todo"

---

<!-- http://0.0.0.0:4446/post/11095385567333215065 -->

# Introduction

When it comes to NixOS systems, a pain point still lives on, in direct conflicts with a *nix* building principle: the `/nix/store`

A public directory where the everything required for the system is stored, where everybody can read what's inside,
when it comes to secret management, this is a real pain in the git.  
A lot of people came up with solutions, and you can see the complete list on the
[dedicated nixos wiki page][secret_mgt_comparison].

However, I'm personally not very satisfied with these solutions as it doesn't match all my personal criteria:
- Have everything stored *inside my git* repository
- Have a very *simple setup* on a new machine (install NixOS base, git clone repo, upgrade system)
- Able to get the secrets from *inside the modules* of my system configuration
- *Easy* to modify said secrets.

All of it while keeping a sufficient security:
- Nothing plaintext inside the `/nix/store` directory
- Secure enough so I can push my secrets online without any fear
- Know where the weak points are, and particular scenarios where to put extra caution

So this is my attempt at a NixOS secrets management system satisfying all of the previously listed points.

> This is heavily based on [Xe's work on a similar subject][xe_article],
check out the post for more details, and many thanks to her for writing it !

This first section is a *simple presentation* for the concept, the following parts are **much more technical** and provide
an example of implementation of such system.

You are advised to be able to read easily NixOS configurations, and data manipulation, *Nix* code in order to go through the
technical parts.

## Presenting the workflow

In this section, I'll detail the "flow of secrets" between the built NixOS system,
the laptop that edited the nix files, and even the remote website hosting the git repository.

Note that I detail *without any particular* order, and leave out details about the implementation
on purpose as everything is explained thoroughly in the next sections.

![The flow chart for the secrets management inside NixOS](/images/attempt_nixos_secrets/secrets_flow.png)[width: "50%", style: "align-self: center"]

> The flow chart for the secrets that I'll cover and detail in the following sections

### In the NixOS system

When configuring NixOS, secrets are generally set using some kind of `services.X.adminPasswordFile = ...` setting.

Because of that, as long as secrets are reachable in the filesystem, everything can work well.  
So the idea is to store everything encrypted inside the `/nix/store`,
and *decrypt them to files* when the system is booting.  
This has the benefit of using the Linux file permissions system to secure who has access to what secrets.

To do all this, we will rely on a single central secret in our system, a RSA private key `/etc/secrets_key`,
secured using **read only permissions to root**, and that will be used at boot to decrypt the secrets.

> We consider that if a hacker *can read* the `/etc/secrets_key` using a vulnerability,
> the system is **already compromised**, and so it doesn't matter if it get access to the system's secrets.  

> The above note is only true if the system doesn't hold secrets that have *a wider scope* (credentials to other systems, etc ...)  
> An extra security layer may be needed in this case.

So the workflow is the following:
- When installing a new NixOS system, you copy the private key used to encrypt all secrets to `/etc/secrets_key`
- You secure the private key by changing the owner to `root:root` and the permissions to read only (400)
- When booting the system, a process will iterate over the encrypted secrets required for the system to work
- Each secret is decrypted (using `rage`), the output is stored in a file (in the `/run/nixos-secrets` directory)
- Each secret is secured with its own owner, group and read/write permissions.

Seams straightforward ! But what about `users.users.<name>.passwordFile` ?  
It requires a specific format (same as `/etc/shadow` ones), how do I hold the encrypted password directly instead of having to transform it before ?

For this, the systemd service will have to perform an *optional transformation phase* on the secret before outputting it to a file.  
In this case, it's hashing the password to a suitable `/etc/shadow` format, but it could be anything depending on the needs.

### In the git repository

In order to configure the system, we need to access the encrypted secrets, and use them to configure services using NixOS modules.
Imagine we have a solution to edit the secrets, that then *outputs a JSON file* containing the secrets data, encrypted using asymmetric cryptography:

``` json
{
  "gitlab": {
    "database_password_file": {
      // Some metadata about the secret
      "age_enc": "BASE64-DATA"
    }
  }
}
```

Then, we can import this data inside a NixOS module using the `builtins.fromJSON` file, and from there we can generated everything !

We just need to generate:
- Services all configured to read their secrets in a file `/run/nixos-secrets/<secret parents>/<secret name>`
- A NixOS activation script that decrypt the secrets using `rage` and output the result in the file
- A systemd service to configure the decrypted secret owner and file permissions

> We cannot set the decrypted secrets owner and file permissions in the activation script because it's executed too early in
the system initialization, and so isn't yet able to it

We use [age](http://age-encryption.org) encryption here as it allows us to encrypt a secret against multiple public keys, so we
don't re-use the same private key for all our NixOS systems.

## Bill of needed software

<!-- TODO This section -->

Talk about to pull this off we need:
- A way to edit manually the secrets, outputting the encrypted JSON file.
- Nix code to generate a NixOS module from the JSON file
- Nix code to generate systemd service from the secrets

# Storing plaintext secrets in the git repository

<!-- TODO This section -->

- Talk about the AES encryption for manual things, and asymmetrical encryption for the provision on the system
- Talk about how the strength of the security here will be the weakest of the 2 (AES / asymmetric), so require strong setup.
- Talk about the piece of software needed to handle this:
  - Encrypted JSON generator from the plaintext JSON (manage_secrets.py)
  - (optional) Plaintext JSON editor, to easily modify things (json_repl)
- Talk about storing public and private keys inside the git repo

<!-- TODO Find a better title for this part -->
# Store encrypted secrets inside the nix store

<!-- TODO This section -->

``` nix
let
  system_secrets = builtins.fromJSON (builtins.readFile ./secrets.json);

  sanitize_name = name: name; # Remove dangerous patterns like path traversal
  secret_path = parents: builtins.concatStringsSep "/" (builtins.map sanitizeName parents);

  generate_secret_option = parents: name: secret: {
    file = lib.mkOption {
      default = "/run/nixos-secrets/${secret_path parents}/${santitize_name name}";
      visible = false;
    };
    enc_data = lib.mkOption {
      # The encrypted data
      default = secret.age_enc;
      visible = false;
    };
  };

  # Goes through the secrets tree and create secrets options if it's a "leaf"
  mkOptionTree = parents: name: data: if builtins.hasAttr "age_enc" data
      then mkSecOptions parents name data
      else lib.attrsets.mapAttrs (mkOptionTree (parents ++ [ name ])) data;
in
  { config, lib, pkgs, ...}: {
    options.secrets.store = mkOptionTree system_secrets;
  }
```

And would allow to configure our system using something like:

``` nix
services.gitlab.databasePasswordFile = config.secrets.store.gitlab.database_password_file.file;
```

This means the configured service will expect the file `/run/nixos-secrets/gitlab/database_password_file` to be present at boot, and readable by the user `gitlab`.

We still have 2 problems:
- We don't want to decrypt every single secrets we have, only the one we need for the system to work
- Secrets requires specific owner / group and permissions in order to be really useful

For this, we will add a new option for our NixOS module:

``` nix
let 
  secretSetupType = lib.types.submodule {
    options = {
      secret = lib.mkOption {
        type = lib.types.attrs;
      };
      user = lib.mkOption {
        type = lib.types.str;
        default = "root";
      };
      group = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = null; # Null = same as user
      };
      perms = lib.mkOption {
        type = lib.types.str;
        default = "400";
      };
    };
  };
in {
  options.secrets.setup = lib.mkOption {
    type = lib.types.attrsOf sectypes.secretSetupType;
    default = {};
  };
}
```

This allows to have an attrsets containing all the secrets to decrypt, and how to configure them.  
It can be used like so:

``` nix
secrets.setup.gitlab_database_password = {
  secret = config.secrets.store.gitlab.database_password_file;
  user = "gitlab";
};
```

Now the last step is to use all this data to decrypt the secrets at boot using the provisioned `/etc/secrets_key`.  
Using a `system.activationScripts`, we can generate the script:

``` nix
system.activationScripts.decrypt_secrets = let
  # Decrypt a secret using the data passed in config.secrets.store
  decrypt_secret = parents: { enc_data, file, ...}: let
    decrypt_cmd = builtins.concatStringsSep " | " [
      "echo \"${age_enc_data}\""
      "base64 -d"
      "${pkgs.rage}/bin/rage --decrypt -i /etc/secrets_key"
    ];
  in ''
    mkdir -p $(dirname ${file})
    ${decrypt_cmd} > ${file}
  '';

  # Traverse the tree of all the enabled secrets, and generate the decryption command
  decrypt_all_secrets = parents: data: if builtins.hasAttr "enc_data" data
    then decrypt_secret parents data
    else lib.attrsets.mapAttrsToList (n: s: decrypt_all_secrets (parents ++ [n]) s) data;

  # Concat all the decrypt commands into a single big script
  decrypt_all_secrets_cmd = builtins.concatStringsSep "\n" (lib.lists.flatten (
    lib.attrsets.mapAttrsToList (n: s: decrypt_all_secrets [n] s.secret) cfg.setup
  ));
in ''
  if [ -f ${cfg.provision_key.file} ]; then
    ${decrypt_all_secrets_cmd}
    echo "All secrets decrypted successfully"
  else
    echo "No provision key set"
  fi
'';
```

When executing an `activationScripts`, the system isn't initialized enough to handle file
permissions. So for now we keep the decrypted files as root-owned, and configure a systemd
service only to change the permissions

``` nix
setup-secrets-perms-and-links = {
  wantedBy = [ "local-fs.target" ];
  after = [ "local-fs.target" ];
  serviceConfig.Type = "oneshot";
  script = let
    # Go through the secrets config and set permissions based on their configuration set
    set_perms_and_link = user: group: perms: data:
      if builtins.hasAttr "__is_leaf" data
      then ''
          if [ -f ${data.file} ]; then
            chmod ${perms} ${data.file}
            chown ${user}:${group} ${data.file}
          else
            echo "File ${data.file} not found, skipping ..."
          fi
        '' + (if builtins.isNull data.link then "" else ''
          ${pkgs.su}/bin/su ${user} -c 'mkdir -p $(dirname ${data.link})'
          rm -f ${data.link}
          ln -s ${data.file} ${data.link}
          chown ${user}:${group} ${data.link}
        '')
      else lib.attrsets.mapAttrsToList (_: d: set_perms_and_link user group perms d) data;

    set_tree_perms_and_link = _: { secret, user, group, perms }: let
      applied_group = if builtins.isNull group then user else group;
    in set_perms_and_link user applied_group perms secret;

 in builtins.concatStringsSep "\n" (
    lib.lists.flatten (lib.attrsets.mapAttrsToList set_tree_perms_and_link cfg.setup)
  );
};
```

# Workflow

## Editing secrets in the git repository

<!-- TODO This section -->

- Talk about how to generate a new key
- Talk about editing secrets manually

## Installing a new system

<!-- TODO This section -->

Using the NixOS official installer, then provide the secret root, then upgrade the system using our dotfiles.

# Assessing this solution

<!-- TODO This section -->

Explain the strengths:
- Configure system with secrets already provided as a module
- Have encrypted secrets inside the store
- Allow to freely modify the secrets manually
- Allow for extended usages (secret files and secrets transformations)

## Weaknesses in configuration

<!-- TODO This section -->

- Problem if /etc/secrets_key not well configured (root read only)
- Problem if unlocked dotfiles are open (TODO Find a linux file permission setup where only user can read / write in the repo)
- Problem if AES encryption for the plaintext secrets uses a weak password

## Weaknesses by vulnerabilities

<!-- TODO This section -->

Problem if following software have vulnerabilities:
- git-crypt
- rage
- AES encryption library

[xe_article]: https://xeiaso.net/blog/nixos-encrypted-secrets-2021-01-20/
[secret_mgt_comparison]: https://nixos.wiki/wiki/Comparison_of_secret_managing_schemes

<!-- TODO Get returns from Xe, Matthias Maschede -->
