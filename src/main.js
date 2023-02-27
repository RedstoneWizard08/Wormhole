const { invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;

function scrollToPage(pageid) {
    window.scrollTo({top: document.getElementById(pageid).offsetTop, behavior: 'smooth'});
}

async function download_file() {
    try {
        const response = await invoke('download_file');
        if (response.toString() === 'exists') {
            scrollToPage('page6');
            alert('File already exists');
            return;
        }
        if (response.toString() === 'notfound') {
            scrollToPage('page5');
            return;
        }
        else {
            scrollToPage('page4');
        }
        console.log(response);
    } catch (error) {
        console.error(error);
    }
}

async function install() {
    const inputPath = document.getElementById("input_path").value;

    if (!inputPath) {
        alert("Please enter a path");
        return;
    }

    console.log(inputPath);

    const response = await invoke('download_file', {kspGivenPath: inputPath});


    if (response === 'exists') {
        alert('File already exists');
        scrollToPage('page6');
        return;
    }
    if (response === 'not-valid') {
        alert('Path is not a valid KSP2 install');
        return;
    }
    if (response === 'notfound') {
        scrollToPage('page5');
        return;
    }
    else {
        scrollToPage('page4');
    }
    console.log(response);
}

async function install_bepinex() {
    const inputPath = document.getElementById("input_path_bep").value;

    if (!inputPath) {
        alert("Please enter a path");
        return;
    }

    console.log(inputPath);

    const response = await invoke('download_bepinex', {kspGivenPath: inputPath});


    if (response === 'exists') {
        alert('File already exists');
        scrollToPage('page6');
        return;
    }
    if (response === 'not-valid') {
        alert('Path is not a valid KSP2 install');
        return;
    }
    if (response === 'notfound') {
        scrollToPage('page5');
        return;
    }
    else {
        scrollToPage('page4');
    }
    console.log(response);
}

async function select_folder() {
    try {
        const result = await open({
            directory: true,
            multiple: false
        });
        if (result === null || result.length === 0) {
            console.log('No folder selected');
            return;
        }
        document.getElementById('input_path').value = result;
    } catch (error) {
        console.error(error);
    }
}

async function select_folder_bep() {
    try {
        const result = await open({
            directory: true,
            multiple: false
        });
        if (result === null || result.length === 0) {
            console.log('No folder selected');
            return;
        }
        document.getElementById('input_path_bep').value = result;
    } catch (error) {
        console.error(error);
    }
}


async function uninstall() {
    try {
        const response = await invoke('uninstall');
        document.getElementById('uninstall_status').style.display = 'block';
        document.getElementById('uninstall_status').innerHTML = response;
        console.log(response);
    } catch (error) {
        console.error(error);
    }
}

async function uninstall_bepinex() {
    try {
        const response = await invoke('uninstall_bepinex');
        console.log(response);
        document.getElementById('uninstall_status_bep').style.display = 'block';
        document.getElementById('uninstall_status_bep').innerHTML = response;
        console.log(response);
    } catch (error) {
        console.error(error);
    }
}


async function download_bepinex_file() {
    try {
        const response = await invoke('download_bepinex');
        if (response.toString() === 'notbep') {
            scrollToPage('page10');
            alert('Not A Valid\nBepInEx Install');
            return;
        }
        if (response.toString() === 'notfound') {
            scrollToPage('page8');
            return;
        }
        else {
            scrollToPage('page4');
        }
        console.log(response);
    } catch (error) {
        console.error(error);
    }
}
