<p align="center"><a href="#" target="_blank" rel="noreferrer noopener"><img width="150" alt="Zeruel Logo" src="https://media.discordapp.net/attachments/590667063165583409/1131711335634321419/Zeruallogoo2.png"></a></p>


<h3 align="center"> Zeruel: Generate QR-Codes & Shorten Links</h3>

<p align="center">Built with Rust & SQLite, Zeruel is a lightweight & easy to use QR-Code generator and URL shortner.</p>
<div align="center">
  <img src="https://github.com/wiki-Bird/Zeruel/actions/workflows/rust.yml/badge.svg">
  
</div>

<br/><br/>
## Installation Guide
### Note: Zeruel is currently in development.

0. Install <a href="https://www.rust-lang.org/learn/get-started" rel="noreferrer noopener">Rust</a> & <a href="https://www.sqlite.org/download.html" rel="noreferrer noopener">SQLite</a>'s "Precompiled Binaries".
   - For more information on installing SQLite, I recommend [this guide]().
2. `git clone https://github.com/wiki-Bird/Zeruel`
3. `cd Zeruel`
4. `cargo build`
5. `cd Zeruel/rust_qr`
6. `diesel migration run`
7. `cargo run`
8. Open `http://localhost:8000` in your browser.

<br/><br/>
## Images


<img width="400" src="https://media.discordapp.net/attachments/590667063165583409/1131709432569548830/image.png">
<img width="400" src="https://media.discordapp.net/attachments/590667063165583409/1132596122242781194/image.png">

<br/><br/>
## Core Dependencies
<ul>
  <li><a href="https://www.rust-lang.org" rel="noreferrer noopener">Rust</a> 1.73.0
  </li>
  <li><a href="https://www.sqlite.org/download.html" rel="noreferrer noopener">SQLite </a>3.42.0</li>
</ul> 
