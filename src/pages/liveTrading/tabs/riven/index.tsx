import { Container } from "@mantine/core";
import { useAppContext } from "@contexts/index";
import { SelectMultipleTradableItems, SettingsForm } from "@components";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";

interface StockRivenPanelProps {
}
export const StockRivenPanel = ({ }: StockRivenPanelProps) => {
    const { settings } = useAppContext();

    const { data } = useQuery({
        queryKey: ['cache_items'],
        queryFn: () => api.cache.getTradableItems(),
    })
    return (
        <Container>
            {settings && (
                <SettingsForm value={settings} onSubmit={() => {
                    console.log('submit');
                }} />
            )}
            <SelectMultipleTradableItems availableItems={data || []} />

        </Container>
    );
};