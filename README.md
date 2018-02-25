Marvin Package Manager for ROS/2
================================

Based on [Cargo](https://crates.io) for Rust.


Goals
------

The aim of this project is two-fold:
1. to converge the ROS community on a central location for finding ROS packages.
2. allow building of packages and dependencies using native toolchains instead of meta-wrappers.


Central Database
----------------

Pre-warning: This is my understanding but I am not an expert in the ROS build/release system and may have some details wrong!

The current method of registering a package within the ROS ecosystem involves the use of the Bloom tool. This is a collection of python scripts that allows developers to:
1. Register a new package with ROS (`distribution.yml`)
2. Create a documentation page on the wiki (???)
3. Define build instructions and create a release of their package (`tracks.yml`)
4. Pull their package dependencies and build their package in CI (`package.xml`)

Together these form a complete dependency resolution system and the instructions for performing all of these actions are improving steadily. So far there are roughly 1,800 packages registered in the central file across 600 repositories.


Dependency Management
---------------------




Why change it?
--------------

1. The current system is targeting a specific pipeline where developers can easily release there packages with minimal effort or knowledge of the underlying tech. This works well for academic researchers where the package isn't the primary goal but it fails to provide the granularity required when using ROS in an industry setting. While possible to clone the build-system used, it was designed to be hosted for free on Github and as the feature set has grown, the complexity of the Bloom tool has grown to where a substantial effort needs to be invested to get the system working and to be sure the source code/binary is safe from accidental public release.
2. As more and more languages are implementing their own package managers, ROS needs to be able to work in symbiosis with them.
3. While not a hard-requirement, creating a single location to register and upload packages makes 

