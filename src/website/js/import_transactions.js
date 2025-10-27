function updateRenderedTransaction() {
  console.log(window.paginationIndex);
  const currentTransaction = window.transactions[window.paginationIndex - 1];
  const vendor = document.getElementById("transaction-vendor");
  const buyer = document.getElementById("transaction-buyer");
  const cost = document.getElementById("transaction-cost");
  const tags = document.getElementById("transaction-tags");
  const date = document.getElementById("transaction-date");

  if (window.prevPaginationIndex) {
    //update json if
    const prevTransaction = window.transactions[window.prevPaginationIndex - 1];
    prevTransaction.vendor = vendor.value;
    prevTransaction.buyer = buyer.value;
    prevTransaction.cost = parseFloat(cost.value);
    prevTransaction.tags = tags.value.split(",").map((t) => {
      return t.trim();
    });
    prevTransaction.date = date.value;

    const rows = document.querySelectorAll("#transaction-items .add-item-row");
    const items = [];

    rows.forEach((row) => {
      const itemName = row.querySelector(".transaction-item-name").value;
      const itemPrice = row.querySelector(".transaction-item-price").value;
      const itemBoughtFor = row.querySelector(
        ".transaction-item-bought-for"
      ).value;
      items.push({
        name: itemName,
        price: parseFloat(itemPrice),
        bought_for: itemBoughtFor,
      });
    });

    prevTransaction.items = items;
  }

  vendor.value = currentTransaction.vendor ? currentTransaction.vendor : null;
  buyer.value = currentTransaction.buyer ? currentTransaction.buyer : null;
  cost.value = currentTransaction.cost ? currentTransaction.cost : null;
  tags.value = currentTransaction.tags ? currentTransaction.tags.join() : null;
  date.value = currentTransaction.date;

  let itemRows = document.getElementsByClassName("add-item-row");
  while (itemRows[0]) {
    itemRows[0].remove();
  }

  fetch("/components/item-row", {
    method: "POST",
    body: JSON.stringify(null),
    headers: {
      authorization: "Bearer " + getCookie("token"),
      "Content-Type": "application/json",
    },
  })
    .then((res) => res.text())
    .then((html) => {
      currentTransaction.items.forEach((item) => {
        let newRow = addHTML(
          html,
          null,
          document.getElementById("transaction-items")
        );
        newRow.querySelector(".transaction-item-name").value = item.name;
        newRow.querySelector(".transaction-item-price").value = item.price;
        newRow.querySelector(".transaction-item-bought-for").value =
          item.bought_for;
      });
    });
}

updateRenderedTransaction();

document
  .querySelector(".pagination-container")
  .addEventListener("click", () => {
    updateRenderedTransaction();
  });

// document.getElementById("remove-transaction").onclick = () => {
//   window.transactions.splice(window.paginationIndex - 1, 1);
//   window.totalPages = window.transactions.length;
//   if (window.paginationIndex > window.transactions.length) {
//     window.paginationIndex = window.transactions.length;
//   }
//   if (window.paginationIndex < 1) {
//     window.paginationIndex = 1;
//   }
//   window.prevPaginationIndex = null;
//   renderPagination();
//   updateRenderedTransaction();
// };

document.getElementById("submit-transactions").onclick = () => {
  // Save changes to current transaction
  window.prevPaginationIndex = window.paginationIndex;
  updateRenderedTransaction();
  let body = JSON.stringify(window.transactions);
  fetch("/api/transactions_many", {
    method: "POST",
    body: body,
    headers: {
      authorization: "Bearer " + getCookie("token"),
      "Content-Type": "application/json",
    },
  }).then(
    () => {
      window.transactions = null;
      window.paginationIndex = null;
      window.prevPaginationIndex = null;
      location.reload();
    },
    (res) => {
      console.log("Failed to add transaction: ", res);
    }
  );
};
