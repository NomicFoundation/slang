contract CrowdFunding {
    struct Campaign {
        address payable beneficiary;
        uint fundingGoal;
    }

    function newCampaign(address payable beneficiary, uint goal) public {
        Campaign storage c;
        c.beneficiary = beneficiary;
        c.fundingGoal = goal;
    }
}
