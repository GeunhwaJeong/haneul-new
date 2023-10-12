import { TransactionBlock } from "@haneullabs/haneul.js/transactions";
import { Button, Container } from "@radix-ui/themes";
import { PACKAGE_ID } from "./constants";
import {
  useSignAndExecuteTransactionBlock,
  useHaneulClient,
} from "@haneullabs/dapp-kit";

export function CreateCounter({
  onCreated,
}: {
  onCreated: (id: string) => void;
}) {
  const client = useHaneulClient();
  const { mutate: signAndExecute } = useSignAndExecuteTransactionBlock();

  return (
    <Container>
      <Button
        size="3"
        onClick={() => {
          create();
        }}
      >
        Create Counter
      </Button>
    </Container>
  );

  function create() {
    const txb = new TransactionBlock();

    txb.moveCall({
      arguments: [],
      target: `${PACKAGE_ID}::counter::create`,
    });

    signAndExecute(
      {
        transactionBlock: txb,
        options: {
          showEffects: true,
          showObjectChanges: true,
        },
      },
      {
        onSuccess: (tx) => {
          client
            .waitForTransactionBlock({
              digest: tx.digest,
            })
            .then(() => {
              const objectId = tx.effects?.created?.[0]?.reference?.objectId;

              if (objectId) {
                onCreated(objectId);
              }
            });
        },
      },
    );
  }
}
