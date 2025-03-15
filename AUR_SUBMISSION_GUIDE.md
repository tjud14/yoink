# Guide to Submitting Your Package to the AUR

This guide will walk you through the process of submitting your Yoink package to the Arch User Repository (AUR).

## Prerequisites

1. An AUR account (create one at https://aur.archlinux.org/)
2. SSH key set up for AUR (add your public key to your AUR account)
3. Git installed on your system
4. The `base-devel` package group installed

## Step 1: Customize the PKGBUILD and .SRCINFO

1. Edit the PKGBUILD file:
   - Replace `Your Name <your.email@example.com>` with your actual name and email
   - Update the GitHub URL to your actual repository URL
   - Verify the version number matches your project's current version

2. Generate an updated .SRCINFO file (if you made changes to PKGBUILD):
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

## Step 2: Test Your Package Locally

1. Build the package locally to ensure it works:
   ```bash
   makepkg -si
   ```

2. Test that the installed program works as expected:
   ```bash
   yoink --help
   ```

## Step 3: Create and Push to the AUR Git Repository

1. Initialize a new Git repository for your AUR package:
   ```bash
   git init
   git add PKGBUILD .SRCINFO
   git commit -m "Initial commit of yoink package"
   ```

2. Add the AUR remote and push your package:
   ```bash
   git remote add aur ssh://aur@aur.archlinux.org/yoink.git
   git push aur master
   ```

## Step 4: Verify Your Package on the AUR

1. Visit https://aur.archlinux.org/packages/yoink to verify your package was uploaded successfully.

2. Users can now install your package using an AUR helper like yay:
   ```bash
   yay -S yoink
   ```

## Maintaining Your Package

1. When you release a new version of your software:
   - Update the `pkgver` in PKGBUILD
   - Update the `.SRCINFO` file
   - Commit and push the changes to the AUR

2. Monitor comments on your AUR package page for user feedback or issues.

## Additional Resources

- [AUR Wiki Page](https://wiki.archlinux.org/title/Arch_User_Repository)
- [PKGBUILD Documentation](https://wiki.archlinux.org/title/PKGBUILD)
- [AUR Submission Guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines) 