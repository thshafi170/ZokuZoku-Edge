import * as vscode from 'vscode';

class Logger {
    private channel: vscode.OutputChannel;

    constructor() {
        this.channel = vscode.window.createOutputChannel("ZokuZoku Debug");
    }

    public log(message: string) {
        this.channel.appendLine(`[${new Date().toISOString()}] [INFO] ${message}`);
    }

    public warn(message: string) {
        this.channel.appendLine(`[${new Date().toISOString()}] [WARN] ${message}`);
    }

    public error(message: string) {
        this.channel.appendLine(`[${new Date().toISOString()}] [ERROR] ${message}`);
        this.channel.show(true);
    }

    public show() {
        this.channel.show(true);
    }
}

export const logger = new Logger();
