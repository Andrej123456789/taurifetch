const invoke = window.__TAURI__.invoke;

OperatingSystem();
Uptime();
Packages();
Shell();
GUI();
CPU();
GPU();

function GetElement_Base(id, function_name) {
  let button_x = document.getElementById(id);
  let substr = button_x.innerHTML.substring(
    button_x.innerHTML.length - 27,
    button_x.innerHTML.length
  );

  button_x.innerHTML = button_x.innerHTML.replace(substr, "");
  invoke(function_name)
    .then((message) => (button_x.innerHTML += message += " " + substr))
    .catch((error) => console.error(error));
}

function OperatingSystem() {
  GetElement_Base("os", "os_name");
}

function Uptime() {
  GetElement_Base("uptime", "uptime");
}

function Packages() {
  GetElement_Base("packages", "packages");
}

function Shell() {
  GetElement_Base("shell", "shell");
}

function GUI() {
  GetElement_Base("gui", "gui");
}

function CPU() {
  GetElement_Base("cpu", "cpu");
}

function GPU() {
  GetElement_Base("gpu", "gpu");
}
