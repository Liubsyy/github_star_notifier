const { invoke } = window.__TAURI__.tauri;

let usernameInput;
let tokenInput;
let periodInput;

async function greet() {

  invoke("toggle_state", { username: usernameInput.value, token:tokenInput.value,period: parseInt(periodInput.value)})
  .then((isRunning) => {
    const button = document.getElementById('startButton');
    if(isRunning){
      button.textContent = '关闭';
      button.classList.remove('btn-start');
      button.classList.add('btn-stop');

      usernameInput.disabled = true;
      tokenInput.disabled = true;
      periodInput.disabled = true;

    }else {
      button.textContent = '开始';
      button.classList.remove('btn-stop');
      button.classList.add('btn-start');

      usernameInput.disabled = false;
      tokenInput.disabled = false;
      periodInput.disabled = false;
     
    }
  })
  .catch((error) => console.error(error))

}

window.addEventListener("DOMContentLoaded", () => {
  usernameInput = document.querySelector("#username");
  tokenInput = document.querySelector("#token");
  periodInput = document.querySelector("#period");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
