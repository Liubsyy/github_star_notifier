const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;
let usernameInput;
let tokenInput;
let periodInput;

function updateInputs(data) {
  usernameInput.value = data.username;
  tokenInput.value = data.token;
  periodInput.value = data.period;
}

async function greet() {

  if(usernameInput.value=='' || periodInput.value==''){
    window.__TAURI__.dialog.message("github用户和频率必须填写","未填写信息");
    return;
  }

  if(!isNaN(parseInt(periodInput.value))) {
    window.__TAURI__.dialog.message("请正确填写频率：秒","填写错误");
    return;
  }



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

  listen('file-data', event => {
    updateInputs(event.payload);
  });

});
