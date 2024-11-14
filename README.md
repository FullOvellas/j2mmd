<!-- Improved compatibility of back to top link: See: https://github.com/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">j2mmd</h3>

  <p align="center">
    Create Mermaid diagrams to document your Java projects.
    <br />
    <br />
    <a href="https://github.com/FullOvellas/j2mmd/issues">Report Bug</a>
    ·
    <a href="https://github.com/FullOvellas/j2mmd/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li>
          <a href="#installation">Installation</a>
          <ul>
            <li><a href="#docker">Docker</a></li>
            <li><a href="#nixos">NixOS</a></li>
          </ul>
        </li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->
## About The Project

This project aims to be a scriptable tool used for assistance when creating documentation for Java
projects.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* [Clap][clap-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

In order to compile and run this project you just need Cargo installed. If you want to use it in a 
Docker container you also need Docker installed in your system.

### Installation

#### Docker

1. Clone the project
   ```sh
   git clone https://github.com/FullOvellas/j2mmd.git
   ```
2. Build the image
   ```sh
   cd j2mmd
   docker build -t j2mmd .
   ```

#### NixOS

You can install this package with Nix if you have the flakes feature enabled.

1. Add this flake as an input to your flake
   ```nix
   {
     #...
     inputs = {
       # ...
       j2mmd.url = "github:FullOvellas/j2mmd"
       #...
     };
     #...
   }
   ```
2. Add the package to your system packages
   ```nix
   environment.systemPackages = with pkgs; [
     # ...
     inputs.j2mmd.packages.${system}.default
     # ...
   ];
   ```
3. Rebuild your system
   ```sh
   sudo nixos-rebuild switch
   ```

Additionally, if you have the nix-command feature enabled, you can get the package in an ephemeral
shell:

```sh
nix shell github:FullOvellas/j2mmd
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

- Create a flowchart diagram for the mappers in the current directory and write it to stdout:
  ```sh
  j2mmd mapper-usage
  ```
- Use additional options to set the directory to analyze or the output file:
  ```sh
  j2mmd mapper-usage -d /path/to/mappers -o ./flowchart.mmd
  ```
- You can use the `help` subcommand on the base command or any subcommands to get more usage info

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

Nothing to see here (for now!).

See the [open issues](https://github.com/FullOvellas/j2mmd/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are welcome!

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open issues.
Don't forget to give the project a star if you found it useful!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: My Amazing Feature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Sergio - [@FullOvellas@mastodon.gal](https://mastodon.gal/@FullOvellas) - alo\_se.r.g.io@proton.me

Project Link: [https://github.com/FullOvellas/j2mmd](https://github.com/FullOvellas/j2mmd)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Choose an Open Source License](https://choosealicense.com)
* [Img Shields](https://shields.io)
* [Mermaid](https://mermaid.js.org/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/FullOvellas/j2mmd.svg?style=for-the-badge
[contributors-url]: https://github.com/FullOvellas/j2mmd/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/FullOvellas/j2mmd.svg?style=for-the-badge
[forks-url]: https://github.com/FullOvellas/j2mmd/network/members
[issues-shield]: https://img.shields.io/github/issues/FullOvellas/j2mmd.svg?style=for-the-badge
[issues-url]: https://github.com/FullOvellas/j2mmd/issues
[license-shield]: https://img.shields.io/github/license/FullOvellas/j2mmd.svg?style=for-the-badge
[license-url]: https://github.com/FullOvellas/j2mmd/blob/main/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/sergio-alonso-dev
[clap-url]: https://docs.rs/clap/latest/clap/index.html
