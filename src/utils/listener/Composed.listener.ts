import { ListenerAction, ListenerRemover, SimpleListener } from "./Simple.listener";


export class ComposedListener {
	private count: number = 0;
	private listeners: Record<string, SimpleListener> = {};

	public getSize = (event?: string | null): number => {
		if (event) {
			const entry = this.listeners[event];
			if (entry) {
				return entry.getSize();
			}
			return 0;
		}
		return this.count;
	};

	public add = (event: string, action: ListenerAction): ListenerRemover => {
		this.count += 1;
		const entry = this.listeners[event] ?? (this.listeners[event] = new SimpleListener());
		const remove = entry.add(action);
		return (): boolean => {
			if (remove()) {
				this.count -= 1;
				if (!entry.getSize()) {
					delete this.listeners[event];
				}
				return true;
			}
			return false;
		};
	};

	public remove = (event: string, action?: ListenerAction): boolean => {
		const entry = this.listeners[event];
		if (entry == null) {
			return false;
		}
		if (action) {
			if (entry.remove(action)) {
				this.count -= 1;
				if (!entry.getSize()) {
					delete this.listeners[event];
				}
				return true;
			}
			return false;
		} else {
			this.count -= entry.getSize();
			delete this.listeners[event];
			return true;
		}
	};

	public fire = (event: string, ...parameters: any[]): void => {
		const entry = this.listeners[event];
		if (entry) {
			entry.fire(...parameters);
		}
	};

	public clean = (event?: string): void => {
		if (event) {
			const entry = this.listeners[event];
			if (entry) {
				entry.clean();
				delete this.listeners[event];
			}
		} else {
			this.count = 0;
			this.listeners = {};
		}
	}
}