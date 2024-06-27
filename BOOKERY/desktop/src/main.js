window.addEventListener("DOMContentLoaded", () => {
  const { invoke } = window.__TAURI__.tauri;
  const tableHead = document.getElementById("table-head");
  const tableBody = document.getElementById("table-body");
  const dispatchSearchURL = {
    Author: "create_table_body_search_author",
    Book: "create_table_body_search_book",
    Costumer: "create_table_body_search_costumer",
    Rental: "create_table_body_search_rental",
  };

  function hideModules() {
    document.querySelectorAll("form").forEach((form) => {
      form.style.display = "none";
    });

    tableHead.innerHTML = "";
    tableBody.innerHTML = "";
  }

  async function createTableHead(module) {
    tableHead.innerHTML = "";
    tableHead.innerHTML = await invoke("create_table_head", {
      module: module,
    });
  }

  async function createTableBodySearch(module) {
    tableBody.innerHTML = "";
    const functionToCall = dispatchSearchURL[module];

    try {
      tableBody.innerHTML = await invoke(functionToCall, {});
    } catch (err) {
      console.log(err);
    }
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
      const module = btn.dataset["module"];

      try {
        createTableHead(module);
        createTableBodySearch(module);
      } catch (err) {
        console.log(err);
      }
    });
  });
});
