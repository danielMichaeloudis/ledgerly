function renderPagination() {
  const pageContainer = document.querySelector(".page-numbers");
  const prevBtn = document.querySelector(".prev");
  const nextBtn = document.querySelector(".next");

  pageContainer.innerHTML = "";

  // Helper: generate visible page numbers with ellipses
  const pages = getVisiblePages(window.paginationIndex, window.totalPages);
  for (const p of pages) {
    if (p === "...") {
      const ellipsis = document.createElement("span");
      ellipsis.classList.add("page-ellipsis");
      ellipsis.textContent = "...";
      pageContainer.appendChild(ellipsis);
    } else {
      const btn = document.createElement("button");
      btn.classList.add("page-number");
      if (p === window.paginationIndex) btn.classList.add("active");
      btn.textContent = p;
      btn.addEventListener("click", () => {
        window.prevPaginationIndex = window.paginationIndex;
        window.paginationIndex = p;
        renderPagination();
      });
      pageContainer.appendChild(btn);
    }
  }

  prevBtn.disabled = window.paginationIndex === 1;
  nextBtn.disabled = window.paginationIndex === window.totalPages;
}

function getVisiblePages(current, total) {
  const pages = [];

  if (total <= 7) {
    for (let i = 1; i <= total; i++) pages.push(i);
  } else {
    const left = Math.max(1, current - 1);
    const right = Math.min(total, current + 1);

    if (left > 2) pages.push(1, "...");
    else for (let i = 1; i < left; i++) pages.push(i);

    for (let i = left; i <= right; i++) pages.push(i);

    if (right < total - 1) pages.push("...", total);
    else for (let i = right + 1; i <= total; i++) pages.push(i);
  }

  return pages;
}

document.querySelector(".prev").addEventListener("click", () => {
  if (window.paginationIndex > 1) {
    window.prevPaginationIndex = window.paginationIndex;
    window.paginationIndex--;
    renderPagination();
  }
});

document.querySelector(".next").addEventListener("click", () => {
  if (window.paginationIndex < window.totalPages) {
    window.prevPaginationIndex = window.paginationIndex;
    window.paginationIndex++;
    renderPagination();
  }
});

renderPagination();
