
<!-- PROJECT SHIELDS -->
<!--

-->




<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/CK-7vn/chipATE">
    <img src="https://github.com/user-attachments/assets/b02c9f2e-5cb9-419c-a181-b6e0eb03b0a2" alt="Logo" width="150" height="200">
  </a>

<h3 align="center">chipATE</h3>

  <p align="center">
    chip8 emulator in the terminal
  </p>



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
        <li><a href="#installation">Installation</a></li>
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
A chip8 emulator TUI, using Ratatui and Rust. There's still some things to workout with this, like the input handling, but for the most part its a fun little project with a couple cool little games to play right in your TUI.
We had to use SDL2 in a tokio thread to handle input handling. Due to how the terminal handles input I was unable to get the KeyRelease, so still brainstorming a way to handle clearing the key state once a key is pressed. 
Was thinking about going a timeout route, but thought that would add some delay. So just for reference SDL2 opens up an essential "hidden" window, that you wont be able to see, but it has to be focused, if the SDL window is not focused your input will not 
be handled, unfortuneately, so that's the next thing on the list to fix. But, it's fun! 

<p align="right">(<a href="#readme-top">back to top</a>)</p>





<!-- GETTING STARTED -->
## Getting Started

Just a simple ```cargo run [romname] ```

<!-- USAGE EXAMPLES -->
## Usage
Pong - 
![image](https://github.com/user-attachments/assets/a7c0c411-c8a7-4246-bc98-a35fd6087bef)
br8kout- 
![image](https://github.com/user-attachments/assets/9a90bde4-eaa8-49cc-83c9-48deab513f0b)
Space invaders - 
![image](https://github.com/user-attachments/assets/f47670c7-0add-4cb7-9db6-6c20d4abbf40)





<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Get key released and remove SDL2
- [ ] Fix some minor bugs



<!-- CONTRIBUTING -->
## Contributing
Please, have at it, lets see how far we can take emulator TUI's to the 🌝







<!-- CONTACT -->
## Contact

Clyde -  keighan@unlockedlabs.org


<p align="right">(<a href="#readme-top">back to top</a>)</p>





<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/github_username/repo_name.svg?style=for-the-badge
[contributors-url]: https://github.com/github_username/repo_name/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/github_username/repo_name.svg?style=for-the-badge
[forks-url]: https://github.com/github_username/repo_name/network/members
[stars-shield]: https://img.shields.io/github/stars/github_username/repo_name.svg?style=for-the-badge
[stars-url]: https://github.com/github_username/repo_name/stargazers
[issues-shield]: https://img.shields.io/github/issues/github_username/repo_name.svg?style=for-the-badge
[issues-url]: https://github.com/github_username/repo_name/issues
[license-shield]: https://img.shields.io/github/license/github_username/repo_name.svg?style=for-the-badge
[license-url]: https://github.com/github_username/repo_name/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 
