const { invoke } = window.__TAURI__.tauri;

function getEmailInputs() {
  const emailInputs = document.querySelectorAll(".email-input");
  return Array.from(emailInputs).map(input => input.value);
}

function addEmailField() {
  const emailFieldsContainer = document.querySelector("#email-fields");
  const newField = document.createElement("div");
  newField.className = "email-field";
  newField.innerHTML = `
    <input type="email" placeholder="Enter email" class="email-input" required />
    <button type="button" class="remove-email-button">✖</button>
  `;
  emailFieldsContainer.appendChild(newField);

  // Add event listener to the remove button
  newField.querySelector(".remove-email-button").addEventListener("click", () => {
    newField.remove();
  });
}

async function sendEmails() {
  const emails = getEmailInputs();
  const htmlFile = document.querySelector("#html-file").files[0];
  
  if (!htmlFile) {
    document.querySelector("#response-msg").textContent = "Please select an HTML file.";
    return;
  }

  const reader = new FileReader();
  reader.onload = async function(event) {
    const htmlContent = event.target.result;

    try {
      await invoke("send_email", { toEmails: emails, htmlContent: htmlContent });
      document.querySelector("#response-msg").textContent = "Emails sent successfully!";
    } catch (error) {
      document.querySelector("#response-msg").textContent = `Failed to send emails: ${error}`;
    }
  };
  reader.readAsText(htmlFile);
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#add-email-button").addEventListener("click", addEmailField);

  document.querySelector("#email-form").addEventListener("submit", (e) => {
    e.preventDefault();
    sendEmails();
  });
});
