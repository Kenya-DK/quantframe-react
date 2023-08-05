class LiveScraper {
  private timer: any;
  private interval: number = 1000;
  public constructor() { }
  public async start() {
    this.timer = setInterval(() => {
      console.log("LiveScraper", Date.now());
    }, this.interval);
  }
  public async stop() {
    clearInterval(this.timer);
  }
}
export const liveScraper = new LiveScraper()
