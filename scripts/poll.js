window.onload = init;

function set_visible(el) {
  el.style.display = 'block';
}

function set_invisible(el) {
  el.style.display = 'none';
}

function invisibleIfChecked(id, parent, hideClass) {
  invisibleIfVal(id, function (t) { return t.checked; }, hideClass);
}

function visibleIfChecked(id, parentId, dispClass) {
  visibleIfVal(id, function (t) {
    let parentq = id;
    if (parentId) {
      parentq = parentId;
    }
    if (!parentq) {
      console.log("ERROR got parentq null: %s %s %s", id, parentId, dispClass);
    }
    let el = document.getElementById(parentq);
    if (el == null) {
      console.log("Can't set %s visible, parent %s not found", dispClass, parentq);
      return false;
    }
    return t.checked && el.checked;
  }, dispClass);
}

function invisibleIfVal(id, check_fct, hideClass) {
  console.log("Add event listener for %s, hide questions %s", id, hideClass);
  add_change_evt(document.getElementById(id), function (evt) {
    if (check_fct(evt.target)) {
      for (q of document.getElementsByClassName(hideClass)) {
        set_invisible(q);
      }
    }
  });
}

function visibleIfVal(id, check_fct, dispClass) {
  console.log("Add event listener for %s, display questions %s", id, dispClass);
  add_change_evt(document.getElementById(id), function (evt) {
    if (check_fct(evt.target)) {
      for (q of document.getElementsByClassName(dispClass)) {
        set_visible(q);
      }
    }
  });
}

function initRangeThreshold(id, min, max, threshold) {
  let arr = Object.entries(threshold).sort((a, b) => {
    if(a[1] > b[1]) return -1;
    if(a[1] < b[1]) return 1;
    return 0
  });

  console.log("ID: %s\nThresholds: %s", id, JSON.stringify(arr));
  var el = document.getElementById(id + "-range");
  el.textContent = el.defaultValue;
  add_change_evt(el, function (evt) {
    let val = evt.target.value;
    var out = document.getElementById(id + "-value");
    var txt = "";
    for (t of arr) {
      if (val <= t[1]) {
        txt = decodeURIComponent(t[0]);
      } else {
        break;
      }
    }
    out.textContent = `${val}/${max} ${txt}`;
  });
}

function add_change_evt(el, handler) {
  var event = new Event('change');
  el.addEventListener("change", handler, false);
  el.dispatchEvent(event);
}

function get_query(q) {
   return (window.location.search.match(new RegExp('[?&]' + q + '=([^&]+)')) || [, null])[1];
}

function init_form_token() {
  let token = get_query("token");
  document.getElementById("save-data-token").value = token;
}

function init() {
  init_form_token();
  init_questions_events();
}
