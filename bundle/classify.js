let toast = false;
let guesses = 0;
const MAX_GUESSES = 5;

function capitalizeFirstLetter(val) {
    return String(val).charAt(0).toUpperCase() + String(val).slice(1);
}

function classify(name, taxonId) {
    guesses++;
    document.getElementById("button").textContent = guesses + '/' + MAX_GUESSES;
    name = name.trim().toLowerCase();
    let genus = document.getElementById("genus").value.trim().toLowerCase();
    let species = document.getElementById("species").value.trim().toLowerCase();
    let [expectedGenus, expectedSpecies] = name.split(" ");

    setBorder(expectedGenus == genus, document.getElementById("genus"));
    setBorder(expectedSpecies == species, document.getElementById("species"));

    if (expectedGenus == genus && expectedSpecies == species) {
        revealToast(name, taxonId);
        document.getElementById("button").setAttribute("disabled", true);
        document.getElementById("result").innerHTML = "<br><span data-testid='result' class='text-emerald-600'>YOU GOT THAT LICHEN!</span>"
    } else {
        if (guesses >= MAX_GUESSES) {
            revealToast(name, taxonId);
            document.getElementById("genus").setAttribute("disabled", true);
            document.getElementById("species").setAttribute("disabled", true);
            document.getElementById("button").setAttribute("disabled", true);
            document.getElementById("result").innerHTML = "<br><span data-testid='result' class='text-red-600'>WRONG!</span>";
        }
    }
}

function revealToast(name, taxonId) {
    if (!toast)  {
        Toastify({destination: "https://www.inaturalist.org/taxa/" + taxonId, position: 'center', newWindow: true, duration: -1, style: { background: "linear-gradient(to right, #006045, #009966)"}, className: "lg:text-base lg:mt-2 text-3xl", text: capitalizeFirstLetter(name)}).showToast();
        toast = true;
    }
}

function setBorder(condition, input) {
    if (condition) {
        input.classList.remove('border-red-600');
        input.classList.add('border-emerald-600');
        input.setAttribute("disabled", true)
    } else {
        input.classList.add('border-red-600');
        input.classList.remove('wrong');
        input.offsetWidth //hacky :)
        input.classList.add('wrong');
    }
}