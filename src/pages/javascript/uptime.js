import { SetSecondaryElement, UpdateClock } from "../../utils.js";

var uptime_done = false;
var epoch_done = false;
var epoch_nice_done = false;

SetSecondaryElement("uptime_two", "uptime");
Epoch();

function Uptime() {
  if (uptime_done = false) {
    SetSecondaryElement("uptime_two", "uptime");
    uptime_done = true;
  } else {
    UpdateClock("Uptime (click for live reloading): ", "uptime_two", "uptime");
  }
}

function Epoch() {
  if (epoch_done = false) {
    SetSecondaryElement("epoch", "epoch");
    epoch_done = true;
  } else {
    UpdateClock(
      "Seconds from epoch (click for live reloading): ",
      "epoch",
      "epoch"
    );
  }
}

function EpochNice() {
  if (epoch_nice_done = false) {
    SetSecondaryElement("epoch_nice", "epoch_nice");
    epoch_nice_done = true;
  } else {
    UpdateClock(
      "Seconds from epoch in nicer way (click for live reloading): ",
      "epoch_nice",
      "epoch_nice"
    );
  }
}

window.Uptime = Uptime;
window.Epoch = Epoch;
window.EpochNice = EpochNice;
