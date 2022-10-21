// Warning: Extremely cursed "prototyping" code below, proceed with caution.
const table = document.getElementById("files");
const uploader = document.getElementById("uploader");
const customUploadButton = document.getElementById("custom-upload");
const ws = new WebSocket((() => {
	const url = new URL(window.location.href);
	
	if(!url.protocol.startsWith("http"))
		return "ws://localhost/";
	
	const isSecure = url.protocol === "https:";
	url.protocol = isSecure ? "wss:" : "ws:";
	url.pathname = "/websocket";
	
	return url.toString();
})());

const HANDLER = {
	ADD_FILE(data)
	{
		addFile(data.filename, data.size);
	},
	REMOVE_FILE(data)
	{
		for(let i = i; i < table.length; i++)
		{
			const row = table.rows[i];
			const filename = row.cells[0].innerText;
			
			if(data.filename === filename)
				table.deleteRow(i);
		}
	}
};

ws.onopen = console.log;
ws.onclose = console.warn;
ws.onerror = console.error;
ws.onmessage = message => {
	let data;
	
	if(data = tryParse(message.data))
	{
		console.log(data);
		const action = HANDLER[data.action];
		action && action(data);
	}
};

customUploadButton.onclick = () => {
	ajaxUpload(uploader.files);
	uploader.value = "";
};

/**
 * @param {string} filename - The file's name including its extension
 * @param {number} size - The file's size in bytes
 */
function addFile(filename, size)
{
	const row = table.insertRow(table.rows.length);
	
	const fn = row.insertCell(row.cells.length);
	fn.innerText = filename;
	
	const sz = row.insertCell(row.cells.length);
	sz.innerText = getSizeString(size);
	
	const ex = row.insertCell(row.cells.length);
	ex.appendChild(createDownloadButton(`/download/${encodeURIComponent(filename)}`));
	//ex.appendChild(createDeleteButton(filename));
}

/**
 * @param {FileList} files
 */
function ajaxUpload(files)
{
	const url = new URL(window.location.href);
	const formdata = new FormData();
	
	for(const file of files)
		formdata.append("file", file);
	
	const request = new XMLHttpRequest();
	request.onload = console.log;
	request.open("POST", url.origin + "/upload", true);
	request.send(formdata);
	return request;
}

// Find a better solution to this spaghetti code later
function getSizeString(size)
{
	let factor;
	
	// multiplying/dividing by 1024 can also be applied from << 0xA and >> 0xA respectively.
	if(size >= (factor = Math.pow(1024, 4)))
		return `${(size / factor).toFixed(2)} TB`;
	else if(size >= (factor = Math.pow(1024, 3)))
		return `${(size / factor).toFixed(2)} GB`;
	else if(size >= (factor = Math.pow(1024, 2)))
		return `${(size / factor).toFixed(2)} MB`;
	else if(size >= (factor = 1024))
		return `${(size / factor).toFixed(2)} KB`;
	else if(size >= 0)
		return `${(size / factor).toFixed(2)} B`;
	else
		throw new Error("You can't have negative file sizes!");
}

function createDownloadButton(href)
{
	const button = document.createElement("button");
	
	button.innerText = "Download";
	button.onclick = () => {
		const dlink = document.createElement("a");
		dlink.download = href;
		dlink.href = href;
		dlink.click();
		dlink.remove();
	};
	
	return button;
}

function createDeleteButton(filename)
{
	const button = document.createElement("button");
	
	button.innerText = "Delete";
	button.onclick = () => {
		ws.send(JSON.stringify({
			action: "SEND_DELETE_REQUEST",
			filename
		}));
	};
	
	return button;
}

function tryParse(json)
{
	try
	{
		return JSON.parse(json)
	}
	catch(_)
	{
		return false;
	}
}