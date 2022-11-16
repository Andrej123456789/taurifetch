import { SetMainElement, SetSecondaryElement } from "./utils.js";

UserComputer();

OperatingSystem();
Uptime();
Packages();
Shell();
GUI();
CPU();
GPU();

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
