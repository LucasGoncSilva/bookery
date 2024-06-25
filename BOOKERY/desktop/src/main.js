window.addEventListener("DOMContentLoaded", () => {
  const { invoke } = window.__TAURI__.tauri;
  const tableHead = document.getElementById("table-head");
  const output = document.getElementById("output");

  function hideForms() {
    document.querySelectorAll("form").forEach((form) => {
      form.style.display = "none";
    });
  }

  async function getDataAPI(module) {
    tableHead.innerHTML = "";
    tableHead.innerHTML = await invoke("create_table_head", {
      module: module,
    });

    output.innerHTML = await invoke("handle_actual_endpoint", {
      module: module,
      action: "Search",
    });
  }

  hideForms();

  // Navbar btns
  document.querySelectorAll(".module-btn").forEach(function (btn) {
    btn.addEventListener("click", function () {
      // Page title
      document.querySelectorAll(".module-name").forEach(function (span) {
        span.textContent = btn.dataset["module"];
      });

      // Show module form
      hideForms();
      document.getElementById(btn.dataset["form"]).style.display =
        "inline-grid";
    });
  });

  document
    .getElementById("author-form-submit")
    .addEventListener("click", function () {
      let name = document.getElementById("author-form-name").value;
      let bornFrom = document.getElementById("author-form-born-from").value;
      let bornUntil = document.getElementById("author-form-born-until").value;

      getDataAPI(this.dataset["module"]);

      output.textContent = name;
    });
});
