// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	toB64,
	toParsedSignaturePubkeyPair,
	getGasData,
	getTransactionSender,
	getTransactionSignature,
	normalizeHaneulAddress,
	type HaneulAddress,
	type HaneulTransactionBlockResponse,
	type SignaturePubkeyPair,
} from '@haneullabs/haneul.js';

import { DescriptionItem, DescriptionList } from '~/ui/DescriptionList';
import { AddressLink } from '~/ui/InternalLink';
import { TabHeader } from '~/ui/Tabs';
import { Text } from '~/ui/Text';

function SignaturePanel({ title, signature }: { title: string; signature: SignaturePubkeyPair }) {
	return (
		<TabHeader title={title}>
			<DescriptionList>
				<DescriptionItem title="Scheme" align="start" labelWidth="sm">
					<Text variant="pBody/medium" color="steel-darker">
						{signature.signatureScheme}
					</Text>
				</DescriptionItem>
				<DescriptionItem title="Address" align="start" labelWidth="sm">
					<AddressLink noTruncate address={signature.pubKey.toHaneulAddress()} />
				</DescriptionItem>
				<DescriptionItem title="Signature" align="start" labelWidth="sm">
					<Text variant="pBody/medium" color="steel-darker">
						{toB64(signature.signature)}
					</Text>
				</DescriptionItem>
			</DescriptionList>
		</TabHeader>
	);
}

function getSignatureFromAddress(signatures: SignaturePubkeyPair[], haneulAddress: HaneulAddress) {
	return signatures.find(
		(signature) => signature.pubKey.toHaneulAddress() === normalizeHaneulAddress(haneulAddress),
	);
}

function getSignaturesExcludingAddress(
	signatures: SignaturePubkeyPair[],
	haneulAddress: HaneulAddress,
): SignaturePubkeyPair[] {
	return signatures.filter(
		(signature) => signature.pubKey.toHaneulAddress() !== normalizeHaneulAddress(haneulAddress),
	);
}
interface Props {
	transaction: HaneulTransactionBlockResponse;
}

export function Signatures({ transaction }: Props) {
	const sender = getTransactionSender(transaction);
	const gasData = getGasData(transaction);
	const transactionSignatures = getTransactionSignature(transaction);

	if (!transactionSignatures) return null;

	const isSponsoredTransaction = gasData?.owner !== sender;

	const deserializedTransactionSignatures = transactionSignatures
		.map((signature) => toParsedSignaturePubkeyPair(signature))
		.flat();

	const userSignatures = isSponsoredTransaction
		? getSignaturesExcludingAddress(deserializedTransactionSignatures, gasData!.owner)
		: deserializedTransactionSignatures;

	const sponsorSignature = isSponsoredTransaction
		? getSignatureFromAddress(deserializedTransactionSignatures, gasData!.owner)
		: null;

	return (
		<div className="flex flex-col gap-8">
			{userSignatures.length > 0 && (
				<div className="flex flex-col gap-8">
					{userSignatures.map((signature, index) => (
						<div key={index}>
							<SignaturePanel title="User Signature" signature={signature} />
						</div>
					))}
				</div>
			)}

			{sponsorSignature && (
				<SignaturePanel title="Sponsor Signature" signature={sponsorSignature} />
			)}
		</div>
	);
}
