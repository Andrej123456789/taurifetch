import { SetMainElement, SetSecondaryElement, ChangeVar } from "./utils.js";

const invoke = window.__TAURI__.invoke;

SetTheme();
Startup();

/* Themes */
function SetTheme() {
  /* console in developers tools said newTheme */
  invoke("theme_name", {
    newTheme: "none",
  })
    .then((message) => {
      document.documentElement.classList.add(message);
    })
    .catch((error) => console.error(error));
}

function ChangeTheme() {
  /* console in developers tools said newTheme */
  invoke("theme_name", {
    newTheme: "none",
  })
    .then((message) => {
      _ChangeTheme(message);
    })
    .catch((error) => console.error(error));
}

function _ChangeTheme(theme) {
  const themes = ["light", "dark"];
  const new_themes = [...themes].filter(function (name) {
    return name !== theme;
  });

  let new_theme = new_themes[Math.floor(Math.random() * new_themes.length)];

  /* console in developers tools said newTheme */
  invoke("theme_name", {
    newTheme: new_theme,
  })
    .then()
    .catch((error) => console.error(error));

  /*switch (new_theme) {
    case "light":
      ChangeVar("prefers-color-scheme", "light");
      break;

    case "dark":
      ChangeVar("prefers-color-scheme", "dark");
      break;
  }*/

  if (document.documentElement.classList.contains("light")) {
    document.documentElement.classList.remove("light");
    document.documentElement.classList.add("dark");
  } else if (document.documentElement.classList.contains("dark")) {
    document.documentElement.classList.remove("dark");
    document.documentElement.classList.add("light");
  } else {
    if (
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches
    ) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.add("light");
    }
  }
}

/* Text */
function Startup() {
  UserComputer();

  OperatingSystem();
  Uptime();
  Packages();
  Shell();
  GUI();
  CPU();
  GPU();
}

function UserComputer() {
  SetSecondaryElement("user", "user_computer");
}

function OperatingSystem() {
  SetMainElement("os", "os_name");
}

function Uptime() {
  SetMainElement("uptime", "uptime");
}

function Packages() {
  SetMainElement("packages", "packages");
}

function Shell() {
  SetMainElement("shell", "shell");
}

function GUI() {
  SetMainElement("gui", "gui");
}

function CPU() {
  SetMainElement("cpu", "cpu");
}

function GPU() {
  SetMainElement("gpu", "gpu");
}

window.ChangeTheme = ChangeTheme;
