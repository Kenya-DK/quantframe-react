import api from '@api/index';
import { SendTauriEvent } from '@utils/index';
import { WTBEntry } from '.';


const sendProgress = (id: string, total: number, current: number, message: string, isCompleted: boolean) => {
  SendTauriEvent("GenerateWtbMessage:Progress", {
    id,
    data: {
      total,
      current,
      isCompleted,
      message
    }
  })
}


export const generateWtbMessage = async (rivenTypes: WTBEntry[], minSellers: number, lowestPrice: number, discount: number) => {
  const rivens: { name: string, url: string, icon: string, sellers: number, lowestPrice: number, previousPrice: number, sellingPrice: number }[] = [];

  for (let index = 0; index < rivenTypes.length; index++) {
    const weapon = rivenTypes[index];
    const auctions = await api.auction.search({
      auction_type: "riven",
      weapon_url_name: weapon.url,
      buyout_policy: "direct",
      polarity: "any",
      sort_by: "price_asc"
    });
    const filtered = auctions.filter(x => x.visible && x.closed == false && x.owner.status == "ingame" && x.is_direct_sell);
    const sellers = filtered.length;
    const lowestPrice = filtered[0]?.buyout_price || 0;
    const rivenPrice = lowestPrice - Math.round(lowestPrice * discount);
    rivens.push({ name: weapon.name, url: weapon.url, icon: weapon.icon, sellers, lowestPrice, sellingPrice: rivenPrice, previousPrice: weapon.price });
    sendProgress("generate-wtb-message", rivenTypes.length, index + 1, `Riven ${weapon.name} has ${sellers} sellers and lowest price is ${lowestPrice}p Total ${rivens.length}/${rivenTypes.length}`, false);

  }
  const filtered = rivens.filter(x => x.sellers >= minSellers && x.lowestPrice >= lowestPrice).sort((a, b) => b.sellingPrice - a.sellingPrice);
  sendProgress("generate-wtb-message", rivenTypes.length, rivenTypes.length, `Done!`, true);
  return filtered.map(x => {
    return {
      url: x.url,
      name: x.name,
      icon: x.icon,
      price: x.sellingPrice,
      // Add Previsous Price
      previousPrice: x.previousPrice,
      hidden: false,
    }
  });
};