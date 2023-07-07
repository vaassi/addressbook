<!-- ABOUT THE PROJECT -->
## AddressBook
Centralized endpoint for storing users contacts.

_Active Directory attributes that app is looking for:_
* "name"
* "employeeNumber"
* "mail"
* "telephoneNumber"
* "mobile"
* "birthdate" (custom attribute)
* "company"
* "department"
* "title"
* "c"
* "postalCode"
* "st"
* "l"
* "streetAddress"

#### Main window
![AddressBook][product-screenshot-1]
#### Details window
![AddressBook][product-screenshot-2]

### Here's why
* Easy installation (standalone binary, docker)
* Cross-platform
* Integrated with Active Directory
* Search autocomplete
* Favorite list
* Birthday indication

### Built With
* [![WebAssemply][WebAssembly.com]][WebAssembly-url]
* [![Bootstrap][Bootstrap.com]][Bootstrap-url]
* [![JQuery][JQuery.com]][JQuery-url]

<!-- GETTING STARTED -->
## Getting Started
To get a local copy up and running follow these simple steps.

### Prerequisites
* Installing Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* Install WebAssembly target
  ```sh
    rustup target add wasm32-unknown-unknown
  ```
* Install Trunk, Make
  ```sh
    cargo install trunk
    cargo install cargo-make
  ```

### Installation
1. Clone the repo
   ```sh
   git clone https://github.com/vaassi/addressbook.git
   ```
2. Create .env file base on .env.example
3. Run cargo-make
   ```sh
   cargo make
   ```
4. Copy in separate folder `./target/release/addressbook.exe`, `static` folder, `.env` file
5. Create assets folder in static dir and put contacts photo base on employeeNumber attribute, f.e. `12345.jpg`
6. Start `addressbook.exe` go to `http://your_url/api/sync` for initial synchronization, wait till get `success`,
this url must be added to cron with schedule you want

### For Docker users
   ```sh
   docker build -t addressbook:0.1.0 .
   ```
   ```sh
    docker run -d --name addressbook --env-file=.env \
    -p 3000:3000 \
    -v ./data/data.sqlite:/opt/app/data.sqlite \
    -v ./data/assets:/opt/app/static/assets \
    vaassi/addressbook:0.1.0
   ```

<!-- CONTRIBUTING -->
## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License
Distributed under the MIT License. See `LICENSE.txt` for more information.

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[product-screenshot-1]: screenshots/screenshot_1.png
[product-screenshot-2]: screenshots/screenshot_2.png

[WebAssembly.com]: https://img.shields.io/badge/webassembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white
[WebAssembly-url]: https://webassembly.org
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com
