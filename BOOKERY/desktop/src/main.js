window.addEventListener("DOMContentLoaded", () => {
  const { invoke } = window.__TAURI__.tauri;
  const tableHead = document.getElementById("table-head");
  const output = document.getElementById("output");

  function hideModules() {
    document.querySelectorAll("form").forEach((form) => {
      form.style.display = "none";
    });

    tableHead.innerHTML = "";
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

  hideModules();

  // Navbar btns
  document.querySelectorAll(".module-btn").forEach(function (btn) {
    btn.addEventListener("click", function () {
      // Page title
      document.querySelectorAll(".module-name").forEach(function (span) {
        span.textContent = btn.dataset["module"];
      });

      // Show module form
      hideModules();
      document.getElementById(btn.dataset["form"]).style.display =
        "inline-grid";
    });
  });

  // Search forms submit
  document.querySelectorAll("form input[type='button']").forEach((btn) => {
    btn.addEventListener("click", () => {
      switch (btn.dataset["module"]) {
        case "Author":
          output.textContent = "Author Switch";
          break;

        case "Book":
          output.textContent = "Book Switch";
          break;

        case "Costumer":
          output.textContent = "Costumer Switch";
          break;

        case "Rental":
          output.textContent = "Rental Switch";
          break;
      }
    });
  });
});
