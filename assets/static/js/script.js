

"use-strict";


const STR_PATTERNS = {
  "location": `${window.location}`,
}


function handleGroupSelector() {

  function switchGroup(group) {

    for (const e of document.querySelectorAll("[data-group]")) {
      e.toggleAttribute("hidden", e.getAttribute("data-group") !== group)
    }
    document.querySelector("header nav li:last-of-type").innerHTML = group;
    document.querySelector("select").value = group;
    localStorage?.setItem("cytt-selected-group", group);
  }

  const select = document.querySelector("select");
  select.addEventListener("change", () => switchGroup(select.value), false);

  const groups = [...document.querySelectorAll("select > option")].map((e) => e.value);
  
  const selectedGroup = localStorage?.getItem("cytt-selected-group");
  if (selectedGroup != null && groups.includes(selectedGroup)) {
    switchGroup(selectedGroup);
  }
}

function handleClipboardCopy() {
  
  for (const e of document.querySelectorAll("[data-copy]")) {
    
    const ogTooltip = e.getAttribute("data-tooltip");
    const data = e.getAttribute("data-copy").replaceAll(/%(.+)%/g, (_, key) => STR_PATTERNS[key]);
    
    document.body.addEventListener("focusout", () => {
      e.setAttribute("data-tooltip", ogTooltip);
      e.classList.remove("success", "error");
    }, false);

    e.addEventListener("click", async () => {
      try {
        
        await navigator.clipboard.writeText(data);
        e.setAttribute("data-tooltip", "Copié!");
        e.classList.add("success");
      
      } catch (err) {
        
        console.error(`ERROR: failed to copy to clipboard;\n${err}`);
        e.setAttribute("data-tooltip", "Echec!");
        e.classList.add("error");

      }
    }, false);
  }
}

function main() {

  handleGroupSelector();

  handleClipboardCopy();

}

document.addEventListener("DOMContentLoaded", main, false);
