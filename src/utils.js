const invoke = window.__TAURI__.invoke;

export function SetMainElement(id, function_name) {
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

export function SetSecondaryElement(id, function_name) {
  let button_x = document.getElementById(id);
  let start_text = button_x.innerHTML;

  invoke(function_name)
    .then((message) => (button_x.innerHTML = start_text + message))
    .catch((error) => console.error(error));
}

export function UpdateClock(base_text, id, function_name) {
  let button_x = document.getElementById(id);

  invoke(function_name)
    .then((message) => (button_x.innerHTML = base_text + message))
    .catch((error) => console.log(error));
}

export function ChangeVar(name, value) {
  document.documentElement.style.setProperty(name, value)
}
