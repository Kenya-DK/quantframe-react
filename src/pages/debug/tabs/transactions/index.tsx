import { SimpleGrid, Image } from "@mantine/core";

interface TransactionPanelProps {
}
export const TransactionPanel = ({ }: TransactionPanelProps) => {
    return (
        <SimpleGrid>
            {Array.from({ length: 10 }).map((_, index) => (
                <Image key={index} src="https://cataas.com/cat" alt="Riven" />
            ))}
        </SimpleGrid>
    );
};