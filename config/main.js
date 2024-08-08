function addOption(select, str) {
    let opt = document.createElement("option");
    opt.value = str;
    opt.innerHTML = str;
    select.appendChild(opt);
}
function addUniqueValue(select, uniqueSet) {
    uniqueSet.forEach((n) => {
        addOption(select, n);
    });
}
function filterRows() {
    const tables = document.getElementsByTagName("table");
    const rows = tables[0].querySelectorAll("tr");
    const selectName = document.getElementById("selName");
    const selectSummary = document.getElementById("selSummary");
    for (let i = 1; i < rows.length; i++) {
        const tdName = rows[i].getElementsByTagName("td")[1];
        const tdSummary = rows[i].getElementsByTagName("td")[2];
        let name = tdName.innerText.split('.').pop();
        let summary = tdSummary.innerText;
        let displayName = false;
        let displaySummary = false;
        if (name.toUpperCase() === selectName.value.toUpperCase() ||
            selectName.value === "all") {
            displayName = true;
        }
        if (summary.toUpperCase() === selectSummary.value.toUpperCase() ||
            selectSummary.value === "all") {
            displaySummary = true;
        }
        if (selectSummary.value === "non-WHQL" &&
            summary.toUpperCase() != "WHQL SIGNED") {
            displaySummary = true;
        }
        if (displayName && displaySummary) {
            rows[i].style.display = "";
        }
        else {
            rows[i].style.display = "none";
        }
    }
}
function toggleColumn(columnName, show) {
    const elements = document.querySelectorAll('.' + columnName);
    elements.forEach(element => {
        element.style.display = show ? '' : 'none';
    });
}
function addCheckbox(columnName, columnClass, isCheck) {
    const displayOptions = document.getElementById('displayOptions');
    const label = document.createElement('label');
    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.id = columnClass;
    checkbox.className = 'toggle-column';
    checkbox.dataset.column = columnName + '-column';
    checkbox.checked = isCheck;
    checkbox.addEventListener('change', function () {
        toggleColumn(columnName + '-column', this.checked);
    });
    label.appendChild(checkbox);
    label.appendChild(document.createTextNode(`${columnName}`));
    displayOptions.appendChild(label);
}
function updateStickyPosition() {
    const rows = document.getElementsByTagName("tr");
    const divFilter = document.getElementById("displayOptions");
    const divFilterHeight = divFilter.getBoundingClientRect().height;
    rows[0].style.top = divFilterHeight + 'px';
    let leftOffset = 0;
    divFilter.childNodes.forEach(label => {
        let labelW = label.getBoundingClientRect().width;
        label.style.left = `${leftOffset}px`;
        leftOffset += labelW;
    });
}
function initColumnResize(col, resizer) {
    let startX = 0;
    let width = 0;
    const mouseDownHandler = function (e) {
        startX = e.clientX;
        const styles = window.getComputedStyle(col);
        width = parseInt(styles.width, 10);
        document.addEventListener('mousemove', resizeColumn);
        document.addEventListener('mouseup', stopResize);
    };
    const resizeColumn = function (e) {
        const dx = e.clientX - startX;
        col.style.width = `${width + dx}px`;
    };
    const stopResize = function () {
        document.removeEventListener('mousemove', resizeColumn);
        document.removeEventListener('mouseup', stopResize);
    };
    resizer.addEventListener('mousedown', mouseDownHandler);
}
function setDisplayOptions() {
    addSingleLineCheckBox();

    const columnsToToggle = ['Path', 'Other', 'Signers'];
    columnsToToggle.forEach(columnName => {
        const columnClass = 'checkbox-' + columnName;
        if (columnName === "Path") {
            addCheckbox(columnName, columnClass, true);
        }
        else {
            addCheckbox(columnName, columnClass, false);
        }
    });

    // Initialize the columns visibility
    document.querySelectorAll('.toggle-column').forEach(checkbox => {
        toggleColumn(checkbox.dataset.column, checkbox.checked);
    });
}
function addOptionsForNameSummary() {
    let head = document.getElementById("Name-column");
    let select = document.createElement("select");
    select.setAttribute("id", "selName");
    select.setAttribute("title", "selName");
    select.addEventListener("change", filterRows);
    head.appendChild(select);
    addOption(select, "all");

    head = document.getElementById("Summary-column");
    select = document.createElement("select");
    select.setAttribute("id", "selSummary");
    select.setAttribute("title", "selSummary");
    select.addEventListener("change", filterRows);
    head.appendChild(select);
    addOption(select, "all");
    addOption(select, "non-WHQL");
}
function addSingleLineCheckBox() {
    const displayOptions = document.getElementById('displayOptions');
    const label = document.createElement('label');
    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.id = "single-line";
    checkbox.className = 'signle-line';
    checkbox.checked = true;
    checkbox.addEventListener('change', function () {
        const resultDiv = document.getElementById("results");
        if (this.checked) {
            resultDiv.style.whiteSpace = "nowrap";
            resultDiv.style.wordBreak = "normal";
        }
        else {
            resultDiv.style.whiteSpace = "normal";
            resultDiv.style.wordBreak = "break-word";
        }
    });
    label.appendChild(checkbox);
    label.appendChild(document.createTextNode("1Line"));
    displayOptions.appendChild(label);
}
function updateStickyColumns() {
    const rows = document.querySelectorAll('tr');
    rows.forEach(row => {
        let leftOffset = 0;
        const cells = row.querySelectorAll('.sticky');
        cells.forEach(cell => {
            cell.style.left = leftOffset + 'px';
            leftOffset += Math.floor(cell.getBoundingClientRect().width) - 0.5;
        });
    });
}
function observeColumnChanges() {
    const table = document.querySelector('table');
    const columns = table.querySelectorAll('th');

    const resizeObserver = new ResizeObserver(entries => {
        for (let entry of entries) {
            updateStickyColumns();
        }
    });

    columns.forEach(column => {
        resizeObserver.observe(column);
    });
}
function setUpUI() {
    addOptionsForNameSummary();
    setDisplayOptions();
    updateStickyPosition();

    let uniqueName = new Set();
    let uniqueSummary = new Set();
    const rows = document.querySelectorAll("tr");
    rows.forEach((row, idx) => {
        if (idx == 0) {
            const columnsToToggle = ['Name', 'Summary', 'Path', 'Other', 'Signers'];
            const headers = row.querySelectorAll('th');
            headers.forEach(header => {
                //add resizer for resizable columns
                const resizer = document.createElement('div');
                resizer.className = 'resizer';
                header.appendChild(resizer);
                initColumnResize(header, resizer);

                //set column width to fit content
                fitWidth = true;
                for (col of columnsToToggle) {
                    if (header.textContent.includes(col)) {
                        fitWidth = false;
                        break;
                    }
                }
                if (fitWidth) {
                    const textLength = header.textContent.trim().length;
                    header.style.width = `${textLength}em`;
                }
                if (header.textContent.includes("Expiry")) {
                    const textLength = header.textContent.trim().length;
                    header.style.width = `${textLength}em`;
                }
            });
        }
        if (idx > 0) {
            //get unique content for filter
            if (row.style.display == "none") {
                return;
            }
            const columns = row.querySelectorAll("td");
            uniqueName.add(columns[1].innerHTML.split('.').pop());
            uniqueSummary.add(columns[2].innerHTML);

            //create tips for overflow content
            const tooltip = document.createElement('div');
            tooltip.className = 'tooltip';
            tooltip.id = 'ellipsis-tip';
            document.body.appendChild(tooltip);
            const oneLineCB = document.getElementById("single-line");
            columns.forEach((cell, idx) => {
                if (cell.classList.length != 0 && idx != 0) {
                    cell.addEventListener('mouseover', function (event) {
                        if (oneLineCB.checked != true) {
                            return;
                        }
                        const content = cell.textContent.trim();
                        if (content.length > 0) {
                            tooltip.textContent = content;
                            tooltip.style.display = 'block';
                            const rect = cell.getBoundingClientRect();
                            tooltip.style.width = `${rect.width - 14}px`;
                            let rectTip = tooltip.getBoundingClientRect();
                            let offsetX = 0;
                            if (rect.width > rectTip.width) {
                                offsetX = (rect.width - rectTip.width) / 2;
                            }
                            let leftVal = rect.left + window.scrollX + offsetX;
                            let topVal = rect.bottom + window.scrollY;
                            tooltip.style.left = `${leftVal}px`;
                            tooltip.style.top = `${topVal}px`;
                            //reload the position of tool tip
                            rectTip = tooltip.getBoundingClientRect();
                            let scrollBarOffset = 30;
                            if (rectTip.right > window.innerWidth) {
                                leftVal -= rectTip.right - window.innerWidth + scrollBarOffset;
                            }
                            if (rectTip.left < 0) {
                                leftVal = window.scrollX;
                            }
                            tooltip.style.left = `${leftVal}px`;
                            if (rectTip.top < 0) {
                                topVal = window.scrollY;
                            }
                            if (rectTip.bottom + scrollBarOffset > window.innerHeight) {
                                topVal = rect.top + window.scrollY - rectTip.height;
                            }
                            tooltip.style.top = `${topVal}px`;
                        }
                    });
                    cell.addEventListener('mouseleave', function () {
                        tooltip.style.display = 'none';
                    });
                }
            });
        }
    });

    //add unique content for filter
    const selectName = document.getElementById("selName");
    const selectSummary = document.getElementById("selSummary");
    addUniqueValue(selectName, uniqueName);
    addUniqueValue(selectSummary, uniqueSummary);
}
document.addEventListener('DOMContentLoaded', function () {
    setUpUI();
    //observeColumnChanges();
});
