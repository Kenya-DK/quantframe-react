// https://kendru.github.io/javascript/2018/12/28/rate-limiting-in-javascript-with-a-token-bucket/
export class TokenBucket {
  private tokens: number
  private lastFilled: number
  constructor(
    private maxBurst: number,
    private fillPerSecond: number
  ) {
    this.lastFilled = Math.floor(Date.now() / 1000)
    this.tokens = maxBurst
  }

  async wait(): Promise<void> {
    this.refill()

    if (this.tokens > 0) {
      this.tokens -= 1
      return
    }

    await new Promise(resolve => setTimeout(resolve, 1000))

    return this.wait()
  }

  private refill() {
    const now = Math.floor(Date.now() / 1000)
    const rate = (now - this.lastFilled) / this.fillPerSecond

    this.tokens = Math.min(this.maxBurst, this.tokens + Math.floor(rate * this.maxBurst))
    this.lastFilled = now
  }
}